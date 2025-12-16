use dioxus_logger::tracing;
use sea_orm::DatabaseConnection;
use serenity::all::GuildId;

use crate::server::{
    data::{
        discord::{DiscordGuildRepository, UserDiscordGuildRepository},
        user::UserRepository,
    },
    error::AppError,
};

pub struct UserDiscordGuildService<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> UserDiscordGuildService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    /// Syncs a user's guild memberships with guilds the bot is present in
    ///
    /// Compares the user's Discord guild memberships with guilds in the database (where the bot is present).
    /// Only creates relationships for guilds where both the user and bot are members. Replaces all existing
    /// guild memberships for the user.
    ///
    /// # Arguments
    /// - `user_id`: Discord user ID (u64)
    /// - `user_guild_ids`: Slice of Discord guild IDs the user is a member of
    ///
    /// # Returns
    /// - `Ok(())`: Sync completed successfully
    /// - `Err(AppError)`: Database error during guild query or sync
    pub async fn sync_user_guilds(
        &self,
        user_id: u64,
        user_guild_ids: &[(GuildId, Option<String>)],
    ) -> Result<(), AppError> {
        let guild_repo = DiscordGuildRepository::new(self.db);
        let user_guild_repo = UserDiscordGuildRepository::new(self.db);

        // Get all guilds the bot is in
        let bot_guilds = guild_repo.get_all().await?;

        // Find matching guilds (where both user and bot are members) with nicknames
        let matching_guilds: Vec<(u64, Option<String>)> = bot_guilds
            .iter()
            .filter_map(|bot_guild| {
                // Parse guild_id from String to u64 for comparison
                if let Ok(guild_id_u64) = bot_guild.guild_id.parse::<u64>() {
                    // Find matching user guild and get nickname
                    user_guild_ids
                        .iter()
                        .find(|(user_guild_id, _)| user_guild_id.get() == guild_id_u64)
                        .map(|(_, nickname)| (guild_id_u64, nickname.clone()))
                } else {
                    None
                }
            })
            .collect();

        // Sync the user's guild memberships
        user_guild_repo
            .sync_user_guilds(user_id, &matching_guilds)
            .await?;

        tracing::debug!(
            "Synced {} guild memberships for user {}",
            matching_guilds.len(),
            user_id
        );

        Ok(())
    }

    /// Syncs members of a guild with logged-in users
    ///
    /// Updates the database to reflect which logged-in users are currently members of the guild.
    /// Removes relationships for users no longer in the guild and adds relationships for new members.
    /// Only processes users who have logged into the application. Used during bot startup to catch
    /// missed member join/leave events while the bot was offline. Updates the last_guild_sync_at
    /// timestamp for all synced users.
    ///
    /// # Arguments
    /// - `guild_id`: Discord's unique identifier for the guild (u64)
    /// - `member_discord_ids`: Slice of Discord user IDs currently in the guild
    ///
    /// # Returns
    /// - `Ok(())`: Sync completed successfully and timestamps updated
    /// - `Err(AppError)`: Database error during user query, guild query, or relationship sync
    pub async fn sync_guild_members(
        &self,
        guild_id: u64,
        member_discord_ids: &[(u64, Option<String>)],
    ) -> Result<(), AppError> {
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

        let guild_repo = DiscordGuildRepository::new(self.db);
        let user_guild_repo = UserDiscordGuildRepository::new(self.db);

        tracing::debug!("Syncing members for guild {}", guild_id);

        // Get the guild from database
        let Some(guild) = guild_repo.find_by_guild_id(guild_id).await? else {
            tracing::warn!(
                "Guild {} not found in database during member sync",
                guild_id
            );
            return Ok(());
        };

        // Get all logged-in users who are members of this Discord guild
        let member_ids: Vec<String> = member_discord_ids
            .iter()
            .map(|(id, _)| id.to_string())
            .collect();

        let logged_in_members: Vec<entity::user::Model> = entity::prelude::User::find()
            .filter(entity::user::Column::DiscordId.is_in(member_ids))
            .all(self.db)
            .await?;

        if logged_in_members.is_empty() {
            tracing::debug!(
                "Found no logged in users for guild {}, nothing to sync",
                guild_id
            );

            // No logged-in users in this guild, nothing to sync
            return Ok(());
        }

        // Get existing relationships for this guild
        let guild_id_u64 = guild
            .guild_id
            .parse::<u64>()
            .map_err(|e| AppError::InternalError(format!("Failed to parse guild_id: {}", e)))?;
        let existing_relationships = user_guild_repo.get_users_by_guild(guild_id_u64).await?;
        let existing_user_ids: std::collections::HashSet<String> = existing_relationships
            .iter()
            .map(|r| r.user_id.clone())
            .collect();

        let logged_in_user_ids: std::collections::HashSet<String> = logged_in_members
            .iter()
            .map(|u| u.discord_id.clone())
            .collect();

        // Collect synced user IDs before moving logged_in_members
        let synced_user_ids: Vec<u64> = logged_in_members
            .iter()
            .filter_map(|u| u.discord_id.parse::<u64>().ok())
            .collect();

        // Remove relationships for users who are no longer in the guild
        for relationship in existing_relationships {
            if !logged_in_user_ids.contains(&relationship.user_id) {
                if let Ok(user_id) = relationship.user_id.parse::<u64>() {
                    user_guild_repo.delete(user_id, guild_id_u64).await?;
                }
            }
        }

        // Add relationships for users who are in the guild but not in our database
        for user in logged_in_members {
            if !existing_user_ids.contains(&user.discord_id) {
                if let Ok(user_id) = user.discord_id.parse::<u64>() {
                    // Find the nickname for this user
                    let nickname = member_discord_ids
                        .iter()
                        .find(|(id, _)| *id == user_id)
                        .and_then(|(_, nick)| nick.clone());
                    user_guild_repo
                        .create(user_id, guild_id_u64, nickname)
                        .await?;
                }
            }
        }

        tracing::info!("Synced members for guild {} ({})", guild.name, guild_id);

        // Update last_guild_sync_at timestamps for all synced users
        if !synced_user_ids.is_empty() {
            let user_repo = UserRepository::new(self.db);
            user_repo
                .update_guild_sync_timestamps(&synced_user_ids)
                .await?;
        }

        Ok(())
    }
}

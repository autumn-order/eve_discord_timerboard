//! Discord guild member service for managing guild membership data.
//!
//! This module provides the `DiscordGuildMemberService` for synchronizing Discord guild
//! member data with the database. It tracks ALL Discord users who are members of guilds
//! the bot has access to, not just users who have logged into the application.

use std::sync::Arc;

use dioxus_logger::tracing;
use sea_orm::DatabaseConnection;
use serenity::all::Http;

use crate::server::{
    data::discord::DiscordGuildMemberRepository, error::AppError,
    service::discord::UserDiscordGuildRoleService,
};

/// Maximum number of members to fetch per API request.
///
/// Discord's API supports up to 1000 members per request. Using the maximum
/// reduces the number of API calls needed for large guilds.
static MEMBERS_PER_REQUEST: u64 = 1000;

/// Service for managing Discord guild member data.
///
/// Provides methods for synchronizing guild membership information from Discord's API
/// to the database. Tracks all Discord users in guilds, handling member additions,
/// updates, and removals. Acts as the orchestration layer between Discord bot events
/// and the member repository.
pub struct DiscordGuildMemberService<'a> {
    /// Database connection for repository operations.
    db: &'a DatabaseConnection,
    discord_http: Arc<Http>,
}

impl<'a> DiscordGuildMemberService<'a> {
    /// Creates a new DiscordGuildMemberService instance.
    ///
    /// # Arguments
    /// - `db` - Reference to the database connection
    ///
    /// # Returns
    /// - `DiscordGuildMemberService` - New service instance
    pub fn new(db: &'a DatabaseConnection, discord_http: Arc<Http>) -> Self {
        Self { db, discord_http }
    }

    /// Syncs all members of a guild with the database.
    ///
    /// Performs a complete sync of guild membership by fetching all members from Discord's API
    /// using pagination (up to 1000 members per request) and updating the database to reflect
    /// the current state. Stores ALL Discord users who are members of the guild, not just users
    /// who have logged into the application. Removes members who have left and adds or updates
    /// current members.
    ///
    /// This method also syncs role assignments for users who have logged into the application,
    /// enabling permission checks based on Discord roles. Members without app accounts are
    /// tracked but do not have role assignments synchronized.
    ///
    /// Requires the `GUILD_MEMBERS` privileged intent to fetch all guild members.
    ///
    /// # Arguments
    /// - `guild_id` - Discord guild ID to sync members for
    ///
    /// # Returns
    /// - `Ok(())` - Sync completed successfully
    /// - `Err(AppError::Database)` - Database error during sync
    /// - `Err(AppError)` - Error fetching members from Discord API
    pub async fn sync_guild_members(&self, guild_id: u64) -> Result<(), AppError> {
        let member_repo = DiscordGuildMemberRepository::new(self.db);

        let mut all_members = Vec::new();
        let mut after: Option<u64> = None;

        // Fetch ALL members from Discord API with pagination
        // This requires the GUILD_MEMBERS privileged intent
        loop {
            match self
                .discord_http
                .get_guild_members(guild_id.into(), Some(MEMBERS_PER_REQUEST), after)
                .await
            {
                Ok(members) => {
                    if members.is_empty() {
                        break;
                    }

                    tracing::trace!(
                        "Fetched {} members from Discord API for guild {} (total so far: {})",
                        members.len(),
                        guild_id,
                        all_members.len() + members.len()
                    );

                    // Set up pagination for next iteration
                    after = members.last().map(|m| m.user.id.get());

                    let fetched_count = members.len();

                    // Add to our collection
                    all_members.extend(members);

                    // If we got less than the maximum, we've reached the end
                    if fetched_count < MEMBERS_PER_REQUEST as usize {
                        break;
                    }
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to fetch guild {} members from API: {:?}",
                        guild_id,
                        e
                    );
                    break;
                }
            }
        }

        // Convert to the format needed for sync: (user_id, username, nickname)
        let member_data: Vec<(u64, String, Option<String>)> = all_members
            .iter()
            .map(|m| (m.user.id.get(), m.user.name.clone(), m.nick.clone()))
            .collect();

        member_repo
            .sync_guild_members(guild_id, &member_data)
            .await?;

        // Sync role memberships for users
        // Only users with app accounts need role assignments for permission checks
        let user_role_service = UserDiscordGuildRoleService::new(self.db);
        if let Err(e) = user_role_service
            .sync_guild_member_roles(guild_id, &all_members)
            .await
        {
            tracing::error!("Failed to sync guild {} member roles: {:?}", guild_id, e);
        }

        tracing::info!(
            "Successfully synced {} members for guild {}",
            member_data.len(),
            guild_id
        );

        Ok(())
    }

    /// Adds or updates a single guild member.
    ///
    /// Creates a new member record or updates an existing member's username and nickname.
    /// Used when handling individual Discord member join or update events from the bot.
    /// Preserves the member's existing data while updating the provided fields.
    ///
    /// # Arguments
    /// - `user_id` - Discord user ID
    /// - `guild_id` - Discord guild ID
    /// - `username` - Discord username
    /// - `nickname` - Optional guild-specific nickname
    ///
    /// # Returns
    /// - `Ok(())` - Member added or updated successfully
    /// - `Err(AppError::Database)` - Database error during upsert
    pub async fn upsert_member(
        &self,
        user_id: u64,
        guild_id: u64,
        username: String,
        nickname: Option<String>,
    ) -> Result<(), AppError> {
        let member_repo = DiscordGuildMemberRepository::new(self.db);
        member_repo
            .upsert(user_id, guild_id, username, nickname)
            .await?;
        Ok(())
    }

    /// Removes a member from a guild.
    ///
    /// Deletes the member record when a user leaves or is removed from the guild.
    /// Used when handling Discord member remove events from the bot. Does not affect
    /// the user's data in other guilds they may be a member of.
    ///
    /// # Arguments
    /// - `user_id` - Discord user ID
    /// - `guild_id` - Discord guild ID
    ///
    /// # Returns
    /// - `Ok(())` - Member removed successfully
    /// - `Err(AppError::Database)` - Database error during deletion
    pub async fn remove_member(&self, user_id: u64, guild_id: u64) -> Result<(), AppError> {
        let member_repo = DiscordGuildMemberRepository::new(self.db);
        member_repo.delete(user_id, guild_id).await?;
        Ok(())
    }
}

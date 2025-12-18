//! Discord guild member service for managing guild membership data.
//!
//! This module provides the `DiscordGuildMemberService` for synchronizing Discord guild
//! member data with the database. It tracks ALL Discord users who are members of guilds
//! the bot has access to, not just users who have logged into the application.

use dioxus_logger::tracing;
use sea_orm::DatabaseConnection;

use crate::server::{data::discord::DiscordGuildMemberRepository, error::AppError};

/// Service for managing Discord guild member data.
///
/// Provides methods for synchronizing guild membership information from Discord's API
/// to the database. Tracks all Discord users in guilds, handling member additions,
/// updates, and removals. Acts as the orchestration layer between Discord bot events
/// and the member repository.
pub struct DiscordGuildMemberService<'a> {
    /// Database connection for repository operations.
    db: &'a DatabaseConnection,
}

impl<'a> DiscordGuildMemberService<'a> {
    /// Creates a new DiscordGuildMemberService instance.
    ///
    /// # Arguments
    /// - `db` - Reference to the database connection
    ///
    /// # Returns
    /// - `DiscordGuildMemberService` - New service instance
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    /// Syncs all members of a guild with the database.
    ///
    /// Performs a complete sync of guild membership by updating the database to reflect
    /// the current state from Discord. Stores ALL Discord users who are members of the
    /// guild, not just users who have logged into the application. Removes members who
    /// have left and adds or updates current members. Used during bot startup and when
    /// significant membership changes occur.
    ///
    /// # Arguments
    /// - `guild_id` - Discord guild ID
    /// - `members` - Slice of tuples containing (user_id, username, nickname) for all members
    ///
    /// # Returns
    /// - `Ok(())` - Sync completed successfully
    /// - `Err(AppError::Database)` - Database error during sync
    pub async fn sync_guild_members(
        &self,
        guild_id: u64,
        members: &[(u64, String, Option<String>)],
    ) -> Result<(), AppError> {
        let member_repo = DiscordGuildMemberRepository::new(self.db);

        tracing::debug!("Syncing {} members for guild {}", members.len(), guild_id);

        member_repo.sync_guild_members(guild_id, members).await?;

        tracing::info!(
            "Successfully synced {} members for guild {}",
            members.len(),
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

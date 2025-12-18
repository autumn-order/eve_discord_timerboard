//! Discord guild service for managing guild data.
//!
//! This module provides the `DiscordGuildService` for retrieving Discord guild
//! information from the database. It handles queries for guild data including
//! names and icons, providing this information to controllers for API responses.

use sea_orm::DatabaseConnection;

use crate::{
    model::discord::DiscordGuildDto,
    server::{data::discord::DiscordGuildRepository, error::AppError},
};

/// Service for managing Discord guild data.
///
/// Provides methods for querying guild information from the database and converting
/// domain models to DTOs for API responses. Acts as the orchestration layer between
/// controllers and the guild repository.
pub struct DiscordGuildService<'a> {
    /// Database connection for repository operations.
    db: &'a DatabaseConnection,
}

impl<'a> DiscordGuildService<'a> {
    /// Creates a new DiscordGuildService instance.
    ///
    /// # Arguments
    /// - `db` - Reference to the database connection
    ///
    /// # Returns
    /// - `DiscordGuildService` - New service instance
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    /// Gets all guilds the bot has access to.
    ///
    /// Retrieves all Discord guilds tracked in the database and converts them to DTOs
    /// for API responses. Returns guild information including ID, name, and icon hash.
    /// Used for displaying guild lists in the UI and guild selection interfaces.
    ///
    /// # Returns
    /// - `Ok(Vec<DiscordGuildDto>)` - List of all guilds with their information
    /// - `Err(AppError::Database)` - Database error during fetch
    pub async fn get_all(&self) -> Result<Vec<DiscordGuildDto>, AppError> {
        let guild_repo = DiscordGuildRepository::new(self.db);

        let guilds: Vec<_> = guild_repo
            .get_all()
            .await?
            .into_iter()
            .map(|g| DiscordGuildDto {
                guild_id: g.guild_id,
                name: g.name,
                icon_hash: g.icon_hash,
            })
            .collect();

        Ok(guilds)
    }

    /// Gets a specific guild by its Discord guild ID.
    ///
    /// Retrieves a single Discord guild from the database and converts it to a DTO
    /// for API responses. Returns guild information including ID, name, and icon hash.
    /// Used for fetching guild details when displaying guild-specific information.
    ///
    /// # Arguments
    /// - `guild_id` - Discord guild ID to retrieve
    ///
    /// # Returns
    /// - `Ok(Some(DiscordGuildDto))` - Guild found with its information
    /// - `Ok(None)` - Guild not found in database
    /// - `Err(AppError::Database)` - Database error during fetch
    pub async fn get_by_guild_id(
        &self,
        guild_id: u64,
    ) -> Result<Option<DiscordGuildDto>, AppError> {
        let guild_repo = DiscordGuildRepository::new(self.db);

        let guild = guild_repo.find_by_guild_id(guild_id).await?;

        Ok(guild.map(|g| DiscordGuildDto {
            guild_id: g.guild_id,
            name: g.name,
            icon_hash: g.icon_hash,
        }))
    }
}

//! Discord guild channel service for managing guild channel synchronization.
//!
//! This module provides the `DiscordGuildChannelService` for synchronizing Discord guild
//! text channels with the database. It filters for text channels only, handles bulk channel
//! updates during bot startup, and provides paginated queries for channel data used in the UI.

use dioxus_logger::tracing;
use sea_orm::DatabaseConnection;
use serenity::all::{ChannelType, GuildChannel};

use crate::{
    model::discord::{DiscordGuildChannelDto, PaginatedDiscordGuildChannelsDto},
    server::{data::discord::DiscordGuildChannelRepository, error::AppError},
};

/// Service for managing Discord guild channels.
///
/// Provides methods for synchronizing text channel data from Discord's API to the database
/// and querying channel information for display in the UI. Only tracks text channels,
/// excluding voice channels, categories, forums, and other channel types. Acts as the
/// orchestration layer between Discord bot events and the channel repository.
pub struct DiscordGuildChannelService<'a> {
    /// Database connection for repository operations.
    db: &'a DatabaseConnection,
}

impl<'a> DiscordGuildChannelService<'a> {
    /// Creates a new DiscordGuildChannelService instance.
    ///
    /// # Arguments
    /// - `db` - Reference to the database connection
    ///
    /// # Returns
    /// - `DiscordGuildChannelService` - New service instance
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    /// Updates channels for a guild by syncing with Discord's current state.
    ///
    /// Performs a complete sync of guild text channels by filtering for text channels only
    /// (excluding voice, category, forum, etc.), deleting channels that no longer exist in
    /// Discord, and upserting all current text channels. This ensures the database accurately
    /// reflects Discord's channel structure. Used during bot startup and when significant
    /// channel changes occur in the guild.
    ///
    /// # Arguments
    /// - `guild_id` - Discord guild ID to update channels for
    /// - `guild_channels` - Slice of all channels in the guild from Discord API
    ///
    /// # Returns
    /// - `Ok(())` - Channels synced successfully
    /// - `Err(AppError::Database)` - Database error during deletion or upsert
    pub async fn update_channels(
        &self,
        guild_id: u64,
        guild_channels: &[GuildChannel],
    ) -> Result<(), AppError> {
        let channel_repo = DiscordGuildChannelRepository::new(self.db);

        // Filter for text channels only
        let text_channels: Vec<&GuildChannel> = guild_channels
            .iter()
            .filter(|channel| channel.kind == ChannelType::Text)
            .collect();

        // Get existing channels from database
        let existing_channels = channel_repo.get_by_guild_id(guild_id).await?;

        // Find channels that no longer exist in Discord and delete them
        for existing_channel in &existing_channels {
            let exists = text_channels
                .iter()
                .any(|channel| channel.id.get() == existing_channel.channel_id);

            if !exists {
                channel_repo.delete(existing_channel.channel_id).await?;
                tracing::info!(
                    "Deleted channel {} from guild {}",
                    existing_channel.channel_id,
                    guild_id
                );
            }
        }

        // Upsert all current text channels
        for channel in &text_channels {
            channel_repo.upsert(guild_id, channel).await?;
        }

        tracing::info!(
            "Updated {} text channels for guild {}",
            text_channels.len(),
            guild_id
        );

        Ok(())
    }

    /// Gets paginated channels for a guild.
    ///
    /// Retrieves a paginated list of text channels for the specified guild, ordered by
    /// position (Discord's channel order). Converts domain models to DTOs for API responses
    /// using offset-based pagination. Used for displaying channel lists in the UI and
    /// channel selection interfaces.
    ///
    /// # Arguments
    /// - `guild_id` - Discord guild ID to fetch channels for
    /// - `page` - Zero-based page number
    /// - `entries` - Number of channels per page
    ///
    /// # Returns
    /// - `Ok(PaginatedDiscordGuildChannelsDto)` - Paginated channel list with metadata
    /// - `Err(AppError::Database)` - Database error during fetch
    pub async fn get_paginated(
        &self,
        guild_id: u64,
        page: u64,
        entries: u64,
    ) -> Result<PaginatedDiscordGuildChannelsDto, AppError> {
        let channel_repo = DiscordGuildChannelRepository::new(self.db);

        // Get all channels for the guild (already sorted by position)
        let all_channels = channel_repo.get_by_guild_id(guild_id).await?;

        // Calculate pagination
        let total = all_channels.len() as u64;
        let start = (page * entries) as usize;

        // Get the page slice and convert to DTOs
        let channel_dtos: Vec<DiscordGuildChannelDto> = all_channels
            .into_iter()
            .skip(start)
            .take(entries as usize)
            .map(|channel| channel.into_dto())
            .collect();

        Ok(PaginatedDiscordGuildChannelsDto {
            channels: channel_dtos,
            total,
            page,
            entries,
        })
    }
}

//! Factory for creating Discord guild channel test data.
//!
//! Provides factory methods for creating Discord guild channels with sensible defaults.
//! Discord guild channels must exist before creating fleet category channels
//! due to foreign key constraints.

use crate::fixture;
use entity::discord_guild_channel;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr};

/// Factory for building Discord guild channel entities with custom values.
///
/// Allows customization of all fields before creation. Use `create_guild_channel()`
/// for quick creation with defaults. Default values are sourced from the
/// discord_guild_channel fixture for consistency across tests.
pub struct DiscordGuildChannelFactory<'a> {
    db: &'a DatabaseConnection,
    entity: discord_guild_channel::Model,
}

impl<'a> DiscordGuildChannelFactory<'a> {
    /// Creates a new factory instance with default values from fixture.
    ///
    /// Defaults are sourced from `fixture::discord_guild_channel::entity()`.
    /// The guild_id and channel_id are set to the provided values.
    ///
    /// # Arguments
    /// - `db` - Database connection for inserting the entity
    /// - `guild_id` - Discord guild ID this channel belongs to
    /// - `channel_id` - Unique Discord channel ID
    pub fn new(db: &'a DatabaseConnection, guild_id: &str, channel_id: &str) -> Self {
        let entity = fixture::discord_guild_channel::entity_builder()
            .guild_id(guild_id)
            .channel_id(channel_id)
            .name(format!("Channel {}", channel_id))
            .build();

        Self { db, entity }
    }

    /// Sets the channel name.
    ///
    /// # Arguments
    /// - `name` - Display name for the channel
    pub fn name(mut self, name: &str) -> Self {
        self.entity.name = name.to_string();
        self
    }

    /// Sets the channel position.
    ///
    /// Lower positions are displayed higher in Discord's channel list.
    ///
    /// # Arguments
    /// - `position` - Channel position value
    pub fn position(mut self, position: i32) -> Self {
        self.entity.position = position;
        self
    }

    /// Builds and inserts the Discord guild channel entity.
    ///
    /// # Returns
    /// - `Ok(Model)` - The created guild channel entity
    /// - `Err(DbErr)` - Database error during insertion
    pub async fn build(self) -> Result<discord_guild_channel::Model, DbErr> {
        discord_guild_channel::ActiveModel {
            guild_id: ActiveValue::Set(self.entity.guild_id),
            channel_id: ActiveValue::Set(self.entity.channel_id),
            name: ActiveValue::Set(self.entity.name),
            position: ActiveValue::Set(self.entity.position),
        }
        .insert(self.db)
        .await
    }
}

/// Creates a Discord guild channel with default values.
///
/// Quick convenience function for creating a guild channel without customization.
/// The channel will have a default name of "Channel {channel_id}" and position 0.
///
/// # Arguments
/// - `db` - Database connection for inserting the entity
/// - `guild_id` - Discord guild ID this channel belongs to
/// - `channel_id` - Unique Discord channel ID
///
/// # Returns
/// - `Ok(Model)` - The created guild channel entity
/// - `Err(DbErr)` - Database error during insertion
///
/// # Example
/// ```rust,ignore
/// let channel = factory::discord_guild_channel::create_guild_channel(&db, &guild.guild_id, "123456").await?;
/// ```
pub async fn create_guild_channel(
    db: &DatabaseConnection,
    guild_id: &str,
    channel_id: &str,
) -> Result<discord_guild_channel::Model, DbErr> {
    DiscordGuildChannelFactory::new(db, guild_id, channel_id)
        .build()
        .await
}

/// Creates a Discord guild channel with custom position.
///
/// Convenience function for creating a guild channel with a specific position.
/// Useful for testing channel sorting behavior.
///
/// # Arguments
/// - `db` - Database connection for inserting the entity
/// - `guild_id` - Discord guild ID this channel belongs to
/// - `channel_id` - Unique Discord channel ID
/// - `position` - Channel position value
///
/// # Returns
/// - `Ok(Model)` - The created guild channel entity
/// - `Err(DbErr)` - Database error during insertion
///
/// # Example
/// ```rust,ignore
/// let top_channel = factory::discord_guild_channel::create_guild_channel_with_position(
///     &db,
///     &guild.guild_id,
///     "123456",
///     1
/// ).await?;
/// ```
pub async fn create_guild_channel_with_position(
    db: &DatabaseConnection,
    guild_id: &str,
    channel_id: &str,
    position: i32,
) -> Result<discord_guild_channel::Model, DbErr> {
    DiscordGuildChannelFactory::new(db, guild_id, channel_id)
        .position(position)
        .build()
        .await
}

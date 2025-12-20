//! Discord guild channel fixtures for creating in-memory test data.
//!
//! Provides fixture functions for creating Discord guild channel entity models without database insertion.
//! These are useful for unit testing, mocking, and providing consistent default values.

use entity::discord_guild_channel;

/// Default test guild ID for channels.
pub const DEFAULT_GUILD_ID: &str = "987654321";

/// Default test channel ID.
pub const DEFAULT_CHANNEL_ID: &str = "111222333";

/// Default test channel name.
pub const DEFAULT_NAME: &str = "Test Channel";

/// Default channel position.
pub const DEFAULT_POSITION: i32 = 0;

/// Creates a Discord guild channel entity model with default values.
///
/// This function creates an in-memory guild channel entity without inserting into the database.
/// Use this for unit tests and mocking repository responses.
///
/// # Default Values
/// - guild_id: `"987654321"`
/// - channel_id: `"111222333"`
/// - name: `"Test Channel"`
/// - position: `0`
///
/// # Returns
/// - `discord_guild_channel::Model` - In-memory guild channel entity
///
/// # Example
///
/// ```rust,ignore
/// use test_utils::fixture;
///
/// let channel = fixture::discord_guild_channel::entity();
/// assert_eq!(channel.name, "Test Channel");
/// assert_eq!(channel.position, 0);
/// ```
pub fn entity() -> discord_guild_channel::Model {
    discord_guild_channel::Model {
        guild_id: DEFAULT_GUILD_ID.to_string(),
        channel_id: DEFAULT_CHANNEL_ID.to_string(),
        name: DEFAULT_NAME.to_string(),
        position: DEFAULT_POSITION,
    }
}

/// Creates a Discord guild channel entity builder for customization.
///
/// Provides a builder pattern for creating guild channel entities with custom values
/// while keeping sensible defaults for unspecified fields.
///
/// # Returns
/// - `DiscordGuildChannelEntityBuilder` - Builder instance with default values
///
/// # Example
///
/// ```rust,ignore
/// use test_utils::fixture;
///
/// let channel = fixture::discord_guild_channel::entity_builder()
///     .channel_id("555666777")
///     .name("General")
///     .position(1)
///     .build();
/// ```
pub fn entity_builder() -> DiscordGuildChannelEntityBuilder {
    DiscordGuildChannelEntityBuilder::default()
}

/// Builder for creating customized Discord guild channel entity models.
///
/// Provides a fluent interface for building guild channel entities with custom values.
/// All fields have sensible defaults that can be overridden.
pub struct DiscordGuildChannelEntityBuilder {
    guild_id: String,
    channel_id: String,
    name: String,
    position: i32,
}

impl Default for DiscordGuildChannelEntityBuilder {
    fn default() -> Self {
        Self {
            guild_id: DEFAULT_GUILD_ID.to_string(),
            channel_id: DEFAULT_CHANNEL_ID.to_string(),
            name: DEFAULT_NAME.to_string(),
            position: DEFAULT_POSITION,
        }
    }
}

impl DiscordGuildChannelEntityBuilder {
    /// Sets the guild ID.
    ///
    /// # Arguments
    /// - `guild_id` - Discord guild ID
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn guild_id(mut self, guild_id: impl Into<String>) -> Self {
        self.guild_id = guild_id.into();
        self
    }

    /// Sets the channel ID.
    ///
    /// # Arguments
    /// - `channel_id` - Discord channel ID
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn channel_id(mut self, channel_id: impl Into<String>) -> Self {
        self.channel_id = channel_id.into();
        self
    }

    /// Sets the channel name.
    ///
    /// # Arguments
    /// - `name` - Display name for the channel
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the channel position.
    ///
    /// # Arguments
    /// - `position` - Channel position value
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn position(mut self, position: i32) -> Self {
        self.position = position;
        self
    }

    /// Builds and returns the Discord guild channel entity model.
    ///
    /// # Returns
    /// - `discord_guild_channel::Model` - In-memory guild channel entity with configured values
    pub fn build(self) -> discord_guild_channel::Model {
        discord_guild_channel::Model {
            guild_id: self.guild_id,
            channel_id: self.channel_id,
            name: self.name,
            position: self.position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_entity_with_defaults() {
        let channel = entity();

        assert_eq!(channel.guild_id, DEFAULT_GUILD_ID);
        assert_eq!(channel.channel_id, DEFAULT_CHANNEL_ID);
        assert_eq!(channel.name, DEFAULT_NAME);
        assert_eq!(channel.position, DEFAULT_POSITION);
    }

    #[test]
    fn builder_creates_entity_with_defaults() {
        let channel = entity_builder().build();

        assert_eq!(channel.guild_id, DEFAULT_GUILD_ID);
        assert_eq!(channel.channel_id, DEFAULT_CHANNEL_ID);
        assert_eq!(channel.name, DEFAULT_NAME);
        assert_eq!(channel.position, DEFAULT_POSITION);
    }

    #[test]
    fn builder_creates_entity_with_custom_values() {
        let channel = entity_builder()
            .guild_id("111222333")
            .channel_id("555666777")
            .name("General")
            .position(5)
            .build();

        assert_eq!(channel.guild_id, "111222333");
        assert_eq!(channel.channel_id, "555666777");
        assert_eq!(channel.name, "General");
        assert_eq!(channel.position, 5);
    }

    #[test]
    fn builder_allows_partial_customization() {
        let channel = entity_builder().name("Announcements").position(1).build();

        assert_eq!(channel.guild_id, DEFAULT_GUILD_ID);
        assert_eq!(channel.channel_id, DEFAULT_CHANNEL_ID);
        assert_eq!(channel.name, "Announcements");
        assert_eq!(channel.position, 1);
    }
}

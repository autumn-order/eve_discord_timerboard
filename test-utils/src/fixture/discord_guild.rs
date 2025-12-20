//! Discord guild fixtures for creating in-memory test data.
//!
//! Provides fixture functions for creating Discord guild entity models without database insertion.
//! These are useful for unit testing, mocking, and providing consistent default values.

use chrono::Utc;
use entity::discord_guild;

/// Default test guild Discord ID.
pub const DEFAULT_GUILD_ID: &str = "987654321";

/// Default test guild name.
pub const DEFAULT_NAME: &str = "Test Guild";

/// Default icon hash for test guilds.
pub const DEFAULT_ICON_HASH: Option<&str> = None;

/// Creates a Discord guild entity model with default values.
///
/// This function creates an in-memory guild entity without inserting into the database.
/// Use this for unit tests and mocking repository responses.
///
/// # Default Values
/// - guild_id: `"987654321"`
/// - name: `"Test Guild"`
/// - icon_hash: `None`
/// - last_sync_at: Current timestamp
///
/// # Returns
/// - `discord_guild::Model` - In-memory guild entity
///
/// # Example
///
/// ```rust,ignore
/// use test_utils::fixture;
///
/// let guild = fixture::discord_guild::entity();
/// assert_eq!(guild.name, "Test Guild");
/// assert!(guild.icon_hash.is_none());
/// ```
pub fn entity() -> discord_guild::Model {
    discord_guild::Model {
        guild_id: DEFAULT_GUILD_ID.to_string(),
        name: DEFAULT_NAME.to_string(),
        icon_hash: DEFAULT_ICON_HASH.map(|s| s.to_string()),
        last_sync_at: Utc::now(),
    }
}

/// Creates a Discord guild entity builder for customization.
///
/// Provides a builder pattern for creating guild entities with custom values
/// while keeping sensible defaults for unspecified fields.
///
/// # Returns
/// - `DiscordGuildEntityBuilder` - Builder instance with default values
///
/// # Example
///
/// ```rust,ignore
/// use test_utils::fixture;
///
/// let custom_guild = fixture::discord_guild::entity_builder()
///     .guild_id("111222333")
///     .name("Custom Guild")
///     .icon_hash(Some("abcd1234".to_string()))
///     .build();
/// ```
pub fn entity_builder() -> DiscordGuildEntityBuilder {
    DiscordGuildEntityBuilder::default()
}

/// Builder for creating customized Discord guild entity models.
///
/// Provides a fluent interface for building guild entities with custom values.
/// All fields have sensible defaults that can be overridden.
pub struct DiscordGuildEntityBuilder {
    guild_id: String,
    name: String,
    icon_hash: Option<String>,
    last_sync_at: chrono::DateTime<Utc>,
}

impl Default for DiscordGuildEntityBuilder {
    fn default() -> Self {
        Self {
            guild_id: DEFAULT_GUILD_ID.to_string(),
            name: DEFAULT_NAME.to_string(),
            icon_hash: DEFAULT_ICON_HASH.map(|s| s.to_string()),
            last_sync_at: Utc::now(),
        }
    }
}

impl DiscordGuildEntityBuilder {
    /// Sets the guild ID.
    ///
    /// # Arguments
    /// - `guild_id` - Discord guild ID as string
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn guild_id(mut self, guild_id: impl Into<String>) -> Self {
        self.guild_id = guild_id.into();
        self
    }

    /// Sets the guild name.
    ///
    /// # Arguments
    /// - `name` - Display name for the guild
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the icon hash.
    ///
    /// # Arguments
    /// - `icon_hash` - Optional Discord icon hash
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn icon_hash(mut self, icon_hash: Option<String>) -> Self {
        self.icon_hash = icon_hash;
        self
    }

    /// Sets the last sync timestamp.
    ///
    /// # Arguments
    /// - `timestamp` - Last sync timestamp
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn last_sync_at(mut self, timestamp: chrono::DateTime<Utc>) -> Self {
        self.last_sync_at = timestamp;
        self
    }

    /// Builds and returns the Discord guild entity model.
    ///
    /// # Returns
    /// - `discord_guild::Model` - In-memory guild entity with configured values
    pub fn build(self) -> discord_guild::Model {
        discord_guild::Model {
            guild_id: self.guild_id,
            name: self.name,
            icon_hash: self.icon_hash,
            last_sync_at: self.last_sync_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_entity_with_defaults() {
        let guild = entity();

        assert_eq!(guild.guild_id, DEFAULT_GUILD_ID);
        assert_eq!(guild.name, DEFAULT_NAME);
        assert!(guild.icon_hash.is_none());
    }

    #[test]
    fn builder_creates_entity_with_defaults() {
        let guild = entity_builder().build();

        assert_eq!(guild.guild_id, DEFAULT_GUILD_ID);
        assert_eq!(guild.name, DEFAULT_NAME);
        assert!(guild.icon_hash.is_none());
    }

    #[test]
    fn builder_creates_entity_with_custom_values() {
        let guild = entity_builder()
            .guild_id("111222333")
            .name("Custom Guild")
            .icon_hash(Some("abcd1234".to_string()))
            .build();

        assert_eq!(guild.guild_id, "111222333");
        assert_eq!(guild.name, "Custom Guild");
        assert_eq!(guild.icon_hash, Some("abcd1234".to_string()));
    }

    #[test]
    fn builder_allows_partial_customization() {
        let guild = entity_builder().name("Partial Guild").build();

        assert_eq!(guild.guild_id, DEFAULT_GUILD_ID);
        assert_eq!(guild.name, "Partial Guild");
        assert!(guild.icon_hash.is_none());
    }
}

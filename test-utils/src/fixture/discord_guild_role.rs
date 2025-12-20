//! Discord guild role fixtures for creating in-memory test data.
//!
//! Provides fixture functions for creating Discord guild role entity models without database insertion.
//! These are useful for unit testing, mocking, and providing consistent default values.

use entity::discord_guild_role;

/// Default test guild ID for roles.
pub const DEFAULT_GUILD_ID: &str = "987654321";

/// Default test role ID.
pub const DEFAULT_ROLE_ID: &str = "444555666";

/// Default test role name.
pub const DEFAULT_NAME: &str = "Test Role";

/// Default role color (empty string).
pub const DEFAULT_COLOR: &str = "";

/// Default role position.
pub const DEFAULT_POSITION: i16 = 0;

/// Creates a Discord guild role entity model with default values.
///
/// This function creates an in-memory guild role entity without inserting into the database.
/// Use this for unit tests and mocking repository responses.
///
/// # Default Values
/// - guild_id: `"987654321"`
/// - role_id: `"444555666"`
/// - name: `"Test Role"`
/// - color: `""` (empty string)
/// - position: `0`
///
/// # Returns
/// - `discord_guild_role::Model` - In-memory guild role entity
///
/// # Example
///
/// ```rust,ignore
/// use test_utils::fixture;
///
/// let role = fixture::discord_guild_role::entity();
/// assert_eq!(role.name, "Test Role");
/// assert_eq!(role.position, 0);
/// ```
pub fn entity() -> discord_guild_role::Model {
    discord_guild_role::Model {
        guild_id: DEFAULT_GUILD_ID.to_string(),
        role_id: DEFAULT_ROLE_ID.to_string(),
        name: DEFAULT_NAME.to_string(),
        color: DEFAULT_COLOR.to_string(),
        position: DEFAULT_POSITION,
    }
}

/// Creates a Discord guild role entity builder for customization.
///
/// Provides a builder pattern for creating guild role entities with custom values
/// while keeping sensible defaults for unspecified fields.
///
/// # Returns
/// - `DiscordGuildRoleEntityBuilder` - Builder instance with default values
///
/// # Example
///
/// ```rust,ignore
/// use test_utils::fixture;
///
/// let role = fixture::discord_guild_role::entity_builder()
///     .role_id("777888999")
///     .name("Admin")
///     .color("#FF0000")
///     .position(10)
///     .build();
/// ```
pub fn entity_builder() -> DiscordGuildRoleEntityBuilder {
    DiscordGuildRoleEntityBuilder::default()
}

/// Builder for creating customized Discord guild role entity models.
///
/// Provides a fluent interface for building guild role entities with custom values.
/// All fields have sensible defaults that can be overridden.
pub struct DiscordGuildRoleEntityBuilder {
    guild_id: String,
    role_id: String,
    name: String,
    color: String,
    position: i16,
}

impl Default for DiscordGuildRoleEntityBuilder {
    fn default() -> Self {
        Self {
            guild_id: DEFAULT_GUILD_ID.to_string(),
            role_id: DEFAULT_ROLE_ID.to_string(),
            name: DEFAULT_NAME.to_string(),
            color: DEFAULT_COLOR.to_string(),
            position: DEFAULT_POSITION,
        }
    }
}

impl DiscordGuildRoleEntityBuilder {
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

    /// Sets the role ID.
    ///
    /// # Arguments
    /// - `role_id` - Discord role ID
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = role_id.into();
        self
    }

    /// Sets the role name.
    ///
    /// # Arguments
    /// - `name` - Display name for the role
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the role color.
    ///
    /// # Arguments
    /// - `color` - Hex color code (e.g., "#FF0000")
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    /// Sets the role position.
    ///
    /// # Arguments
    /// - `position` - Role position value
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn position(mut self, position: i16) -> Self {
        self.position = position;
        self
    }

    /// Builds and returns the Discord guild role entity model.
    ///
    /// # Returns
    /// - `discord_guild_role::Model` - In-memory guild role entity with configured values
    pub fn build(self) -> discord_guild_role::Model {
        discord_guild_role::Model {
            guild_id: self.guild_id,
            role_id: self.role_id,
            name: self.name,
            color: self.color,
            position: self.position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_entity_with_defaults() {
        let role = entity();

        assert_eq!(role.guild_id, DEFAULT_GUILD_ID);
        assert_eq!(role.role_id, DEFAULT_ROLE_ID);
        assert_eq!(role.name, DEFAULT_NAME);
        assert_eq!(role.color, DEFAULT_COLOR);
        assert_eq!(role.position, DEFAULT_POSITION);
    }

    #[test]
    fn builder_creates_entity_with_defaults() {
        let role = entity_builder().build();

        assert_eq!(role.guild_id, DEFAULT_GUILD_ID);
        assert_eq!(role.role_id, DEFAULT_ROLE_ID);
        assert_eq!(role.name, DEFAULT_NAME);
        assert_eq!(role.color, DEFAULT_COLOR);
        assert_eq!(role.position, DEFAULT_POSITION);
    }

    #[test]
    fn builder_creates_entity_with_custom_values() {
        let role = entity_builder()
            .guild_id("111222333")
            .role_id("777888999")
            .name("Admin")
            .color("#FF0000")
            .position(10)
            .build();

        assert_eq!(role.guild_id, "111222333");
        assert_eq!(role.role_id, "777888999");
        assert_eq!(role.name, "Admin");
        assert_eq!(role.color, "#FF0000");
        assert_eq!(role.position, 10);
    }

    #[test]
    fn builder_allows_partial_customization() {
        let role = entity_builder().name("Moderator").color("#00FF00").build();

        assert_eq!(role.guild_id, DEFAULT_GUILD_ID);
        assert_eq!(role.role_id, DEFAULT_ROLE_ID);
        assert_eq!(role.name, "Moderator");
        assert_eq!(role.color, "#00FF00");
        assert_eq!(role.position, DEFAULT_POSITION);
    }
}

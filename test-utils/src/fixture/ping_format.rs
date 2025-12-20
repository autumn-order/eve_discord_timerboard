//! Ping format fixtures for creating in-memory test data.
//!
//! Provides fixture functions for creating ping format entity models without database insertion.
//! These are useful for unit testing, mocking, and providing consistent default values.

use entity::ping_format;

/// Default test ping format name.
pub const DEFAULT_NAME: &str = "Test Ping Format";

/// Default test guild ID for ping formats.
pub const DEFAULT_GUILD_ID: &str = "987654321";

/// Creates a ping format entity model with default values.
///
/// This function creates an in-memory ping format entity without inserting into the database.
/// Use this for unit tests and mocking repository responses.
///
/// # Default Values
/// - id: `1`
/// - guild_id: `"987654321"`
/// - name: `"Test Ping Format"`
///
/// # Returns
/// - `ping_format::Model` - In-memory ping format entity
///
/// # Example
///
/// ```rust,ignore
/// use test_utils::fixture;
///
/// let ping_format = fixture::ping_format::entity();
/// assert_eq!(ping_format.name, "Test Ping Format");
/// ```
pub fn entity() -> ping_format::Model {
    ping_format::Model {
        id: 1,
        guild_id: DEFAULT_GUILD_ID.to_string(),
        name: DEFAULT_NAME.to_string(),
    }
}

/// Creates a ping format entity builder for customization.
///
/// Provides a builder pattern for creating ping format entities with custom values
/// while keeping sensible defaults for unspecified fields.
///
/// # Returns
/// - `PingFormatEntityBuilder` - Builder instance with default values
///
/// # Example
///
/// ```rust,ignore
/// use test_utils::fixture;
///
/// let custom_format = fixture::ping_format::entity_builder()
///     .guild_id("111222333")
///     .name("Custom Format")
///     .build();
/// ```
pub fn entity_builder() -> PingFormatEntityBuilder {
    PingFormatEntityBuilder::default()
}

/// Builder for creating customized ping format entity models.
///
/// Provides a fluent interface for building ping format entities with custom values.
/// All fields have sensible defaults that can be overridden.
pub struct PingFormatEntityBuilder {
    id: i32,
    guild_id: String,
    name: String,
}

impl Default for PingFormatEntityBuilder {
    fn default() -> Self {
        Self {
            id: 1,
            guild_id: DEFAULT_GUILD_ID.to_string(),
            name: DEFAULT_NAME.to_string(),
        }
    }
}

impl PingFormatEntityBuilder {
    /// Sets the ping format ID.
    ///
    /// # Arguments
    /// - `id` - Ping format ID
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

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

    /// Sets the ping format name.
    ///
    /// # Arguments
    /// - `name` - Display name for the ping format
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Builds and returns the ping format entity model.
    ///
    /// # Returns
    /// - `ping_format::Model` - In-memory ping format entity with configured values
    pub fn build(self) -> ping_format::Model {
        ping_format::Model {
            id: self.id,
            guild_id: self.guild_id,
            name: self.name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_entity_with_defaults() {
        let ping_format = entity();

        assert_eq!(ping_format.id, 1);
        assert_eq!(ping_format.guild_id, DEFAULT_GUILD_ID);
        assert_eq!(ping_format.name, DEFAULT_NAME);
    }

    #[test]
    fn builder_creates_entity_with_defaults() {
        let ping_format = entity_builder().build();

        assert_eq!(ping_format.id, 1);
        assert_eq!(ping_format.guild_id, DEFAULT_GUILD_ID);
        assert_eq!(ping_format.name, DEFAULT_NAME);
    }

    #[test]
    fn builder_creates_entity_with_custom_values() {
        let ping_format = entity_builder()
            .id(5)
            .guild_id("111222333")
            .name("Custom Format")
            .build();

        assert_eq!(ping_format.id, 5);
        assert_eq!(ping_format.guild_id, "111222333");
        assert_eq!(ping_format.name, "Custom Format");
    }

    #[test]
    fn builder_allows_partial_customization() {
        let ping_format = entity_builder().name("Partial Format").build();

        assert_eq!(ping_format.id, 1);
        assert_eq!(ping_format.guild_id, DEFAULT_GUILD_ID);
        assert_eq!(ping_format.name, "Partial Format");
    }
}

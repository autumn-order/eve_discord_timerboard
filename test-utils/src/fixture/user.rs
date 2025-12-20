//! User fixtures for creating in-memory test data.
//!
//! Provides fixture functions for creating user entity models without database insertion.
//! These are useful for unit testing, mocking, and providing consistent default values.

use chrono::Utc;
use entity::user;

/// Default test user Discord ID.
pub const DEFAULT_DISCORD_ID: &str = "123456789";

/// Default test user name.
pub const DEFAULT_NAME: &str = "Test User";

/// Default admin status for test users.
pub const DEFAULT_ADMIN: bool = false;

/// Creates a user entity model with default values.
///
/// This function creates an in-memory user entity without inserting into the database.
/// Use this for unit tests and mocking repository responses.
///
/// # Default Values
/// - discord_id: `"123456789"`
/// - name: `"Test User"`
/// - admin: `false`
/// - last_guild_sync_at: Current timestamp
/// - last_role_sync_at: Current timestamp
///
/// # Returns
/// - `user::Model` - In-memory user entity
///
/// # Example
///
/// ```rust,ignore
/// use test_utils::fixture;
///
/// let user = fixture::user::entity();
/// assert_eq!(user.name, "Test User");
/// assert!(!user.admin);
/// ```
pub fn entity() -> user::Model {
    let now = Utc::now();
    user::Model {
        discord_id: DEFAULT_DISCORD_ID.to_string(),
        name: DEFAULT_NAME.to_string(),
        admin: DEFAULT_ADMIN,
        last_guild_sync_at: now,
        last_role_sync_at: now,
    }
}

/// Creates a user entity builder for customization.
///
/// Provides a builder pattern for creating user entities with custom values
/// while keeping sensible defaults for unspecified fields.
///
/// # Returns
/// - `UserEntityBuilder` - Builder instance with default values
///
/// # Example
///
/// ```rust,ignore
/// use test_utils::fixture;
///
/// let admin = fixture::user::entity_builder()
///     .discord_id("987654321")
///     .name("Admin User")
///     .admin(true)
///     .build();
/// ```
pub fn entity_builder() -> UserEntityBuilder {
    UserEntityBuilder::default()
}

/// Builder for creating customized user entity models.
///
/// Provides a fluent interface for building user entities with custom values.
/// All fields have sensible defaults that can be overridden.
pub struct UserEntityBuilder {
    discord_id: String,
    name: String,
    admin: bool,
    last_guild_sync_at: chrono::DateTime<Utc>,
    last_role_sync_at: chrono::DateTime<Utc>,
}

impl Default for UserEntityBuilder {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            discord_id: DEFAULT_DISCORD_ID.to_string(),
            name: DEFAULT_NAME.to_string(),
            admin: DEFAULT_ADMIN,
            last_guild_sync_at: now,
            last_role_sync_at: now,
        }
    }
}

impl UserEntityBuilder {
    /// Sets the Discord ID.
    ///
    /// # Arguments
    /// - `discord_id` - Discord user ID as string
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn discord_id(mut self, discord_id: impl Into<String>) -> Self {
        self.discord_id = discord_id.into();
        self
    }

    /// Sets the user name.
    ///
    /// # Arguments
    /// - `name` - Display name for the user
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the admin status.
    ///
    /// # Arguments
    /// - `admin` - Whether the user has admin privileges
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn admin(mut self, admin: bool) -> Self {
        self.admin = admin;
        self
    }

    /// Sets the last guild sync timestamp.
    ///
    /// # Arguments
    /// - `timestamp` - Last sync timestamp
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn last_guild_sync_at(mut self, timestamp: chrono::DateTime<Utc>) -> Self {
        self.last_guild_sync_at = timestamp;
        self
    }

    /// Sets the last role sync timestamp.
    ///
    /// # Arguments
    /// - `timestamp` - Last sync timestamp
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn last_role_sync_at(mut self, timestamp: chrono::DateTime<Utc>) -> Self {
        self.last_role_sync_at = timestamp;
        self
    }

    /// Builds and returns the user entity model.
    ///
    /// # Returns
    /// - `user::Model` - In-memory user entity with configured values
    pub fn build(self) -> user::Model {
        user::Model {
            discord_id: self.discord_id,
            name: self.name,
            admin: self.admin,
            last_guild_sync_at: self.last_guild_sync_at,
            last_role_sync_at: self.last_role_sync_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_entity_with_defaults() {
        let user = entity();

        assert_eq!(user.discord_id, DEFAULT_DISCORD_ID);
        assert_eq!(user.name, DEFAULT_NAME);
        assert_eq!(user.admin, DEFAULT_ADMIN);
    }

    #[test]
    fn builder_creates_entity_with_defaults() {
        let user = entity_builder().build();

        assert_eq!(user.discord_id, DEFAULT_DISCORD_ID);
        assert_eq!(user.name, DEFAULT_NAME);
        assert!(!user.admin);
    }

    #[test]
    fn builder_creates_entity_with_custom_values() {
        let user = entity_builder()
            .discord_id("987654321")
            .name("Custom User")
            .admin(true)
            .build();

        assert_eq!(user.discord_id, "987654321");
        assert_eq!(user.name, "Custom User");
        assert!(user.admin);
    }

    #[test]
    fn builder_allows_partial_customization() {
        let user = entity_builder().admin(true).build();

        assert_eq!(user.discord_id, DEFAULT_DISCORD_ID);
        assert_eq!(user.name, DEFAULT_NAME);
        assert!(user.admin);
    }
}

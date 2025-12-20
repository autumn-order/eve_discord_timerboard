//! Fleet fixtures for creating in-memory test data.
//!
//! Provides fixture functions for creating fleet entity models without database insertion.
//! These are useful for unit testing, mocking, and providing consistent default values.

use chrono::Utc;
use entity::fleet;

/// Default test fleet name.
pub const DEFAULT_NAME: &str = "Test Fleet";

/// Default test fleet description.
pub const DEFAULT_DESCRIPTION: Option<&str> = Some("Test fleet description");

/// Default hidden status for test fleets.
pub const DEFAULT_HIDDEN: bool = false;

/// Default reminder disabled status for test fleets.
pub const DEFAULT_DISABLE_REMINDER: bool = false;

/// Creates a fleet entity model with default values.
///
/// This function creates an in-memory fleet entity without inserting into the database.
/// Use this for unit tests and mocking repository responses.
///
/// # Default Values
/// - id: `1`
/// - category_id: `1`
/// - name: `"Test Fleet"`
/// - commander_id: `"123456789"`
/// - fleet_time: 1 hour from now
/// - description: `Some("Test fleet description")`
/// - hidden: `false`
/// - disable_reminder: `false`
/// - created_at: Current timestamp
///
/// # Returns
/// - `fleet::Model` - In-memory fleet entity
///
/// # Example
///
/// ```rust,ignore
/// use test_utils::fixture;
///
/// let fleet = fixture::fleet::entity();
/// assert_eq!(fleet.name, "Test Fleet");
/// assert!(!fleet.hidden);
/// ```
pub fn entity() -> fleet::Model {
    let now = Utc::now();
    fleet::Model {
        id: 1,
        category_id: 1,
        name: DEFAULT_NAME.to_string(),
        commander_id: "123456789".to_string(),
        fleet_time: now + chrono::Duration::hours(1),
        description: DEFAULT_DESCRIPTION.map(|s| s.to_string()),
        hidden: DEFAULT_HIDDEN,
        disable_reminder: DEFAULT_DISABLE_REMINDER,
        created_at: now,
    }
}

/// Creates a fleet entity builder for customization.
///
/// Provides a builder pattern for creating fleet entities with custom values
/// while keeping sensible defaults for unspecified fields.
///
/// # Returns
/// - `FleetEntityBuilder` - Builder instance with default values
///
/// # Example
///
/// ```rust,ignore
/// use test_utils::fixture;
///
/// let fleet = fixture::fleet::entity_builder()
///     .name("Custom Fleet")
///     .commander_id("987654321")
///     .hidden(true)
///     .build();
/// ```
pub fn entity_builder() -> FleetEntityBuilder {
    FleetEntityBuilder::default()
}

/// Builder for creating customized fleet entity models.
///
/// Provides a fluent interface for building fleet entities with custom values.
/// All fields have sensible defaults that can be overridden.
pub struct FleetEntityBuilder {
    id: i32,
    category_id: i32,
    name: String,
    commander_id: String,
    fleet_time: chrono::DateTime<Utc>,
    description: Option<String>,
    hidden: bool,
    disable_reminder: bool,
    created_at: chrono::DateTime<Utc>,
}

impl Default for FleetEntityBuilder {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: 1,
            category_id: 1,
            name: DEFAULT_NAME.to_string(),
            commander_id: "123456789".to_string(),
            fleet_time: now + chrono::Duration::hours(1),
            description: DEFAULT_DESCRIPTION.map(|s| s.to_string()),
            hidden: DEFAULT_HIDDEN,
            disable_reminder: DEFAULT_DISABLE_REMINDER,
            created_at: now,
        }
    }
}

impl FleetEntityBuilder {
    /// Sets the fleet ID.
    ///
    /// # Arguments
    /// - `id` - Fleet ID
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn id(mut self, id: i32) -> Self {
        self.id = id;
        self
    }

    /// Sets the category ID.
    ///
    /// # Arguments
    /// - `category_id` - Fleet category ID
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn category_id(mut self, category_id: i32) -> Self {
        self.category_id = category_id;
        self
    }

    /// Sets the fleet name.
    ///
    /// # Arguments
    /// - `name` - Display name for the fleet
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the commander ID.
    ///
    /// # Arguments
    /// - `commander_id` - Discord ID of the fleet commander
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn commander_id(mut self, commander_id: impl Into<String>) -> Self {
        self.commander_id = commander_id.into();
        self
    }

    /// Sets the fleet time.
    ///
    /// # Arguments
    /// - `fleet_time` - Scheduled time for the fleet
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn fleet_time(mut self, fleet_time: chrono::DateTime<Utc>) -> Self {
        self.fleet_time = fleet_time;
        self
    }

    /// Sets the fleet description.
    ///
    /// # Arguments
    /// - `description` - Optional fleet description
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    /// Sets whether the fleet is hidden.
    ///
    /// # Arguments
    /// - `hidden` - Whether the fleet should be hidden
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Sets whether reminders are disabled.
    ///
    /// # Arguments
    /// - `disable_reminder` - Whether to disable reminders
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn disable_reminder(mut self, disable_reminder: bool) -> Self {
        self.disable_reminder = disable_reminder;
        self
    }

    /// Sets the created timestamp.
    ///
    /// # Arguments
    /// - `timestamp` - Creation timestamp
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn created_at(mut self, timestamp: chrono::DateTime<Utc>) -> Self {
        self.created_at = timestamp;
        self
    }

    /// Builds and returns the fleet entity model.
    ///
    /// # Returns
    /// - `fleet::Model` - In-memory fleet entity with configured values
    pub fn build(self) -> fleet::Model {
        fleet::Model {
            id: self.id,
            category_id: self.category_id,
            name: self.name,
            commander_id: self.commander_id,
            fleet_time: self.fleet_time,
            description: self.description,
            hidden: self.hidden,
            disable_reminder: self.disable_reminder,
            created_at: self.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_entity_with_defaults() {
        let fleet = entity();

        assert_eq!(fleet.name, DEFAULT_NAME);
        assert_eq!(
            fleet.description,
            DEFAULT_DESCRIPTION.map(|s| s.to_string())
        );
        assert_eq!(fleet.hidden, DEFAULT_HIDDEN);
        assert_eq!(fleet.disable_reminder, DEFAULT_DISABLE_REMINDER);
    }

    #[test]
    fn builder_creates_entity_with_defaults() {
        let fleet = entity_builder().build();

        assert_eq!(fleet.name, DEFAULT_NAME);
        assert!(!fleet.hidden);
        assert!(!fleet.disable_reminder);
    }

    #[test]
    fn builder_creates_entity_with_custom_values() {
        let custom_time = Utc::now() + chrono::Duration::hours(2);
        let fleet = entity_builder()
            .name("Custom Fleet")
            .commander_id("987654321")
            .fleet_time(custom_time)
            .description(Some("Custom description".to_string()))
            .hidden(true)
            .disable_reminder(true)
            .build();

        assert_eq!(fleet.name, "Custom Fleet");
        assert_eq!(fleet.commander_id, "987654321");
        assert_eq!(fleet.fleet_time, custom_time);
        assert_eq!(fleet.description, Some("Custom description".to_string()));
        assert!(fleet.hidden);
        assert!(fleet.disable_reminder);
    }

    #[test]
    fn builder_allows_partial_customization() {
        let fleet = entity_builder().name("Partial Fleet").hidden(true).build();

        assert_eq!(fleet.name, "Partial Fleet");
        assert!(fleet.hidden);
        assert!(!fleet.disable_reminder);
        assert_eq!(
            fleet.description,
            DEFAULT_DESCRIPTION.map(|s| s.to_string())
        );
    }
}

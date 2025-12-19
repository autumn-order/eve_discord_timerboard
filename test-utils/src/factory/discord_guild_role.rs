//! Factory for creating Discord guild role test data.
//!
//! Provides factory methods for creating Discord guild roles with sensible defaults.
//! Discord guild roles must exist before creating fleet category access roles or ping roles
//! due to foreign key constraints.

use entity::discord_guild_role;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr};

/// Factory for building Discord guild role entities with custom values.
///
/// Allows customization of all fields before creation. Use `create_guild_role()`
/// for quick creation with defaults.
pub struct DiscordGuildRoleFactory<'a> {
    db: &'a DatabaseConnection,
    guild_id: String,
    role_id: String,
    name: Option<String>,
    color: String,
    position: i16,
}

impl<'a> DiscordGuildRoleFactory<'a> {
    /// Creates a new factory instance with required fields.
    ///
    /// # Arguments
    /// - `db` - Database connection for inserting the entity
    /// - `guild_id` - Discord guild ID this role belongs to
    /// - `role_id` - Unique Discord role ID
    pub fn new(db: &'a DatabaseConnection, guild_id: &str, role_id: &str) -> Self {
        Self {
            db,
            guild_id: guild_id.to_string(),
            role_id: role_id.to_string(),
            name: None,
            color: String::new(),
            position: 0,
        }
    }

    /// Sets the role name.
    ///
    /// # Arguments
    /// - `name` - Display name for the role
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Sets the role color.
    ///
    /// # Arguments
    /// - `color` - Hex color code (e.g., "#FF0000")
    pub fn color(mut self, color: &str) -> Self {
        self.color = color.to_string();
        self
    }

    /// Sets the role position.
    ///
    /// Higher positions are displayed higher in Discord's role list.
    ///
    /// # Arguments
    /// - `position` - Role position value
    pub fn position(mut self, position: i16) -> Self {
        self.position = position;
        self
    }

    /// Builds and inserts the Discord guild role entity.
    ///
    /// # Returns
    /// - `Ok(Model)` - The created guild role entity
    /// - `Err(DbErr)` - Database error during insertion
    pub async fn build(self) -> Result<discord_guild_role::Model, DbErr> {
        let name = self
            .name
            .unwrap_or_else(|| format!("Role {}", self.role_id));

        discord_guild_role::ActiveModel {
            guild_id: ActiveValue::Set(self.guild_id),
            role_id: ActiveValue::Set(self.role_id),
            name: ActiveValue::Set(name),
            color: ActiveValue::Set(self.color),
            position: ActiveValue::Set(self.position),
        }
        .insert(self.db)
        .await
    }
}

/// Creates a Discord guild role with default values.
///
/// Quick convenience function for creating a guild role without customization.
/// The role will have a default name of "Role {role_id}", empty color, and position 0.
///
/// # Arguments
/// - `db` - Database connection for inserting the entity
/// - `guild_id` - Discord guild ID this role belongs to
/// - `role_id` - Unique Discord role ID
///
/// # Returns
/// - `Ok(Model)` - The created guild role entity
/// - `Err(DbErr)` - Database error during insertion
///
/// # Example
/// ```rust,ignore
/// let role = factory::discord_guild_role::create_guild_role(&db, &guild.guild_id, "123456").await?;
/// ```
pub async fn create_guild_role(
    db: &DatabaseConnection,
    guild_id: &str,
    role_id: &str,
) -> Result<discord_guild_role::Model, DbErr> {
    DiscordGuildRoleFactory::new(db, guild_id, role_id)
        .build()
        .await
}

/// Creates a Discord guild role with custom position.
///
/// Convenience function for creating a guild role with a specific position.
/// Useful for testing role sorting behavior.
///
/// # Arguments
/// - `db` - Database connection for inserting the entity
/// - `guild_id` - Discord guild ID this role belongs to
/// - `role_id` - Unique Discord role ID
/// - `position` - Role position value
///
/// # Returns
/// - `Ok(Model)` - The created guild role entity
/// - `Err(DbErr)` - Database error during insertion
///
/// # Example
/// ```rust,ignore
/// let high_role = factory::discord_guild_role::create_guild_role_with_position(
///     &db,
///     &guild.guild_id,
///     "123456",
///     10
/// ).await?;
/// ```
pub async fn create_guild_role_with_position(
    db: &DatabaseConnection,
    guild_id: &str,
    role_id: &str,
    position: i16,
) -> Result<discord_guild_role::Model, DbErr> {
    DiscordGuildRoleFactory::new(db, guild_id, role_id)
        .position(position)
        .build()
        .await
}

//! User factory for creating test user entities.
//!
//! This module provides factory methods for creating user entities with sensible
//! defaults, reducing boilerplate in tests. The factory supports customization
//! through a builder pattern.

use crate::factory::helpers::next_id;
use crate::fixture;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr};

/// Factory for creating test users with customizable fields.
///
/// Provides a builder pattern for creating user entities with default values
/// that can be overridden as needed for specific test scenarios. Default values
/// are sourced from the user fixture for consistency across tests.
///
/// # Example
///
/// ```rust,ignore
/// use test_utils::factory::user::UserFactory;
///
/// let user = UserFactory::new(&db)
///     .discord_id("123456789")
///     .name("CustomUser")
///     .admin(true)
///     .build()
///     .await?;
/// ```
pub struct UserFactory<'a> {
    db: &'a DatabaseConnection,
    entity: entity::user::Model,
}

impl<'a> UserFactory<'a> {
    /// Creates a new UserFactory with default values from fixture.
    ///
    /// Defaults are sourced from `fixture::user::entity()` with a unique
    /// auto-incremented ID to prevent conflicts when creating multiple users.
    ///
    /// # Arguments
    /// - `db` - Database connection for inserting the entity
    ///
    /// # Returns
    /// - `UserFactory` - New factory instance with defaults
    pub fn new(db: &'a DatabaseConnection) -> Self {
        let id = next_id();
        let entity = fixture::user::entity_builder()
            .discord_id(id.to_string())
            .name(format!("User {}", id))
            .build();

        Self { db, entity }
    }

    /// Sets the Discord ID for the user.
    ///
    /// # Arguments
    /// - `discord_id` - Discord user ID as string
    ///
    /// # Returns
    /// - `Self` - Factory instance for method chaining
    pub fn discord_id(mut self, discord_id: impl Into<String>) -> Self {
        self.entity.discord_id = discord_id.into();
        self
    }

    /// Sets the name for the user.
    ///
    /// # Arguments
    /// - `name` - Display name for the user
    ///
    /// # Returns
    /// - `Self` - Factory instance for method chaining
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.entity.name = name.into();
        self
    }

    /// Sets the admin status for the user.
    ///
    /// # Arguments
    /// - `admin` - Whether the user should have admin privileges
    ///
    /// # Returns
    /// - `Self` - Factory instance for method chaining
    pub fn admin(mut self, admin: bool) -> Self {
        self.entity.admin = admin;
        self
    }

    /// Builds and inserts the user entity into the database.
    ///
    /// # Returns
    /// - `Ok(entity::user::Model)` - Created user entity
    /// - `Err(DbErr)` - Database error during insert
    pub async fn build(self) -> Result<entity::user::Model, DbErr> {
        entity::user::ActiveModel {
            discord_id: ActiveValue::Set(self.entity.discord_id),
            name: ActiveValue::Set(self.entity.name),
            admin: ActiveValue::Set(self.entity.admin),
            last_guild_sync_at: ActiveValue::Set(self.entity.last_guild_sync_at),
            last_role_sync_at: ActiveValue::Set(self.entity.last_role_sync_at),
        }
        .insert(self.db)
        .await
    }
}

/// Creates a user with default values.
///
/// Shorthand for `UserFactory::new(db).build().await`.
///
/// # Arguments
/// - `db` - Database connection
///
/// # Returns
/// - `Ok(entity::user::Model)` - Created user entity
/// - `Err(DbErr)` - Database error during insert
///
/// # Example
///
/// ```rust,ignore
/// let user = create_user(&db).await?;
/// ```
pub async fn create_user(db: &DatabaseConnection) -> Result<entity::user::Model, DbErr> {
    UserFactory::new(db).build().await
}

/// Creates a user with a specific numeric Discord ID.
///
/// Shorthand for `UserFactory::new(db).discord_id(discord_id).build().await`.
///
/// # Arguments
/// - `db` - Database connection
/// - `discord_id` - Discord ID as string or number
///
/// # Returns
/// - `Ok(entity::user::Model)` - Created user entity
/// - `Err(DbErr)` - Database error during insert
///
/// # Example
///
/// ```rust,ignore
/// let user = create_user(db, "123456789").await?;
/// ```
pub async fn create_user_with_id(
    db: &DatabaseConnection,
    discord_id: impl Into<String>,
) -> Result<entity::user::Model, DbErr> {
    UserFactory::new(db).discord_id(discord_id).build().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::TestBuilder;
    use entity::prelude::*;

    #[tokio::test]
    async fn creates_user_with_defaults() -> Result<(), DbErr> {
        let test = TestBuilder::new().with_table(User).build().await.unwrap();
        let db = test.db.as_ref().unwrap();

        let user = create_user(db).await?;

        assert!(!user.discord_id.is_empty());
        assert!(!user.name.is_empty());
        assert!(!user.admin);

        Ok(())
    }

    #[tokio::test]
    async fn creates_user_with_custom_values() -> Result<(), DbErr> {
        let test = TestBuilder::new().with_table(User).build().await.unwrap();
        let db = test.db.as_ref().unwrap();

        let user = UserFactory::new(db)
            .discord_id("123456789")
            .name("CustomUser")
            .admin(true)
            .build()
            .await?;

        assert_eq!(user.discord_id, "123456789");
        assert_eq!(user.name, "CustomUser");
        assert!(user.admin);

        Ok(())
    }

    #[tokio::test]
    async fn creates_multiple_unique_users() -> Result<(), DbErr> {
        let test = TestBuilder::new().with_table(User).build().await.unwrap();
        let db = test.db.as_ref().unwrap();

        let user1 = create_user(db).await?;
        let user2 = create_user(db).await?;

        assert_ne!(user1.discord_id, user2.discord_id);
        assert_ne!(user1.name, user2.name);

        Ok(())
    }
}

//! Factory for creating Discord guild member test data.
//!
//! Provides factory methods for creating Discord guild members with sensible defaults.
//! Guild members must have existing users and guilds due to foreign key constraints.

use crate::fixture;
use entity::discord_guild_member;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr};

/// Factory for building Discord guild member entities with custom values.
///
/// Allows customization of all fields before creation. Use `create_guild_member()`
/// for quick creation with defaults. Default values are sourced from the
/// discord_guild_member fixture for consistency across tests.
pub struct DiscordGuildMemberFactory<'a> {
    db: &'a DatabaseConnection,
    entity: discord_guild_member::Model,
}

impl<'a> DiscordGuildMemberFactory<'a> {
    /// Creates a new factory instance with default values from fixture.
    ///
    /// Defaults are sourced from `fixture::discord_guild_member::entity()`.
    /// The user_id and guild_id are set to the provided values.
    ///
    /// # Arguments
    /// - `db` - Database connection for inserting the entity
    /// - `user_id` - Discord user ID
    /// - `guild_id` - Discord guild ID this member belongs to
    pub fn new(db: &'a DatabaseConnection, user_id: u64, guild_id: u64) -> Self {
        let entity = fixture::discord_guild_member::entity_builder()
            .user_id(user_id.to_string())
            .guild_id(guild_id.to_string())
            .username(format!("User {}", user_id))
            .build();

        Self { db, entity }
    }

    /// Sets the username.
    ///
    /// # Arguments
    /// - `username` - Discord username
    pub fn username(mut self, username: &str) -> Self {
        self.entity.username = username.to_string();
        self
    }

    /// Sets the nickname.
    ///
    /// # Arguments
    /// - `nickname` - Guild-specific nickname
    pub fn nickname(mut self, nickname: Option<&str>) -> Self {
        self.entity.nickname = nickname.map(|n| n.to_string());
        self
    }

    /// Builds and inserts the Discord guild member entity.
    ///
    /// # Returns
    /// - `Ok(Model)` - The created guild member entity
    /// - `Err(DbErr)` - Database error during insertion
    pub async fn build(self) -> Result<discord_guild_member::Model, DbErr> {
        discord_guild_member::ActiveModel {
            user_id: ActiveValue::Set(self.entity.user_id),
            guild_id: ActiveValue::Set(self.entity.guild_id),
            username: ActiveValue::Set(self.entity.username),
            nickname: ActiveValue::Set(self.entity.nickname),
        }
        .insert(self.db)
        .await
    }
}

/// Creates a Discord guild member with default values.
///
/// Quick convenience function for creating a guild member without customization.
/// The member will have a default username of "User {user_id}" and no nickname.
///
/// # Arguments
/// - `db` - Database connection for inserting the entity
/// - `user_id` - Discord user ID
/// - `guild_id` - Discord guild ID
///
/// # Returns
/// - `Ok(Model)` - The created guild member entity
/// - `Err(DbErr)` - Database error during insertion
///
/// # Example
/// ```rust,ignore
/// let member = factory::discord_guild_member::create_guild_member(&db, 123456789, 987654321).await?;
/// ```
pub async fn create_guild_member(
    db: &DatabaseConnection,
    user_id: u64,
    guild_id: u64,
) -> Result<discord_guild_member::Model, DbErr> {
    DiscordGuildMemberFactory::new(db, user_id, guild_id)
        .build()
        .await
}

/// Creates a Discord guild member with a nickname.
///
/// Convenience function for creating a guild member with a guild-specific nickname.
///
/// # Arguments
/// - `db` - Database connection for inserting the entity
/// - `user_id` - Discord user ID
/// - `guild_id` - Discord guild ID
/// - `nickname` - Guild-specific nickname
///
/// # Returns
/// - `Ok(Model)` - The created guild member entity
/// - `Err(DbErr)` - Database error during insertion
///
/// # Example
/// ```rust,ignore
/// let member = factory::discord_guild_member::create_guild_member_with_nickname(
///     &db,
///     123456789,
///     987654321,
///     "Cool Nickname"
/// ).await?;
/// ```
pub async fn create_guild_member_with_nickname(
    db: &DatabaseConnection,
    user_id: u64,
    guild_id: u64,
    nickname: &str,
) -> Result<discord_guild_member::Model, DbErr> {
    DiscordGuildMemberFactory::new(db, user_id, guild_id)
        .nickname(Some(nickname))
        .build()
        .await
}

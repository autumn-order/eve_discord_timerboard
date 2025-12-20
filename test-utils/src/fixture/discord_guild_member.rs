//! Fixture for Discord guild member test data.
//!
//! Provides fixture methods for creating in-memory guild member data
//! without database insertion. Used for unit testing and mocking.

use entity::discord_guild_member;

/// Default test user Discord ID.
pub const DEFAULT_USER_ID: &str = "123456789";

/// Default test guild Discord ID.
pub const DEFAULT_GUILD_ID: &str = "987654321";

/// Default test username.
pub const DEFAULT_USERNAME: &str = "TestUser";

/// Creates a Discord guild member entity model with default values.
///
/// Returns an in-memory entity without database insertion.
/// User ID defaults to "123456789", guild ID defaults to "987654321",
/// username defaults to "TestUser", and nickname is None.
///
/// # Returns
/// - `discord_guild_member::Model` - In-memory entity
///
/// # Example
/// ```rust,ignore
/// let member = fixture::discord_guild_member::entity();
/// assert_eq!(member.username, "TestUser");
/// ```
pub fn entity() -> discord_guild_member::Model {
    discord_guild_member::Model {
        user_id: DEFAULT_USER_ID.to_string(),
        guild_id: DEFAULT_GUILD_ID.to_string(),
        username: DEFAULT_USERNAME.to_string(),
        nickname: None,
    }
}

/// Creates a customizable Discord guild member entity builder.
///
/// Use this when you need to override default values.
///
/// # Returns
/// - `DiscordGuildMemberEntityBuilder` - Builder with default values
///
/// # Example
/// ```rust,ignore
/// let member = fixture::discord_guild_member::entity_builder()
///     .user_id("111111111")
///     .guild_id("222222222")
///     .username("CustomUser")
///     .nickname(Some("CoolNick"))
///     .build();
/// ```
pub fn entity_builder() -> DiscordGuildMemberEntityBuilder {
    DiscordGuildMemberEntityBuilder::default()
}

/// Builder for Discord guild member entity models.
///
/// Creates customizable entity models without database insertion.
/// All fields have sensible defaults for testing.
pub struct DiscordGuildMemberEntityBuilder {
    user_id: String,
    guild_id: String,
    username: String,
    nickname: Option<String>,
}

impl Default for DiscordGuildMemberEntityBuilder {
    fn default() -> Self {
        Self {
            user_id: DEFAULT_USER_ID.to_string(),
            guild_id: DEFAULT_GUILD_ID.to_string(),
            username: DEFAULT_USERNAME.to_string(),
            nickname: None,
        }
    }
}

impl DiscordGuildMemberEntityBuilder {
    /// Sets the user ID.
    ///
    /// # Arguments
    /// - `user_id` - Discord user ID
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
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

    /// Sets the username.
    ///
    /// # Arguments
    /// - `username` - Discord username
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = username.into();
        self
    }

    /// Sets the nickname.
    ///
    /// # Arguments
    /// - `nickname` - Optional guild-specific nickname
    ///
    /// # Returns
    /// - `Self` - Builder instance for method chaining
    pub fn nickname(mut self, nickname: Option<impl Into<String>>) -> Self {
        self.nickname = nickname.map(|n| n.into());
        self
    }

    /// Builds the entity model.
    ///
    /// # Returns
    /// - `discord_guild_member::Model` - In-memory entity with configured values
    pub fn build(self) -> discord_guild_member::Model {
        discord_guild_member::Model {
            user_id: self.user_id,
            guild_id: self.guild_id,
            username: self.username,
            nickname: self.nickname,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_entity_with_defaults() {
        let member = entity();

        assert_eq!(member.user_id, DEFAULT_USER_ID);
        assert_eq!(member.guild_id, DEFAULT_GUILD_ID);
        assert_eq!(member.username, DEFAULT_USERNAME);
        assert!(member.nickname.is_none());
    }

    #[test]
    fn builder_creates_entity_with_defaults() {
        let member = entity_builder().build();

        assert_eq!(member.user_id, DEFAULT_USER_ID);
        assert_eq!(member.guild_id, DEFAULT_GUILD_ID);
        assert_eq!(member.username, DEFAULT_USERNAME);
        assert!(member.nickname.is_none());
    }

    #[test]
    fn builder_creates_entity_with_custom_values() {
        let member = entity_builder()
            .user_id("111111111")
            .guild_id("222222222")
            .username("CustomUser")
            .nickname(Some("CoolNick"))
            .build();

        assert_eq!(member.user_id, "111111111");
        assert_eq!(member.guild_id, "222222222");
        assert_eq!(member.username, "CustomUser");
        assert_eq!(member.nickname, Some("CoolNick".to_string()));
    }

    #[test]
    fn builder_allows_partial_customization() {
        let member = entity_builder().username("PartialUser").build();

        assert_eq!(member.user_id, DEFAULT_USER_ID);
        assert_eq!(member.guild_id, DEFAULT_GUILD_ID);
        assert_eq!(member.username, "PartialUser");
        assert!(member.nickname.is_none());
    }
}

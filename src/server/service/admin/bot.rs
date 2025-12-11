use oauth2::{CsrfToken, Scope};
use sea_orm::DatabaseConnection;
use serenity::all::Permissions;
use url::Url;

use crate::server::{
    data::user::UserRepository,
    error::{auth::AuthError, AppError},
    state::OAuth2Client,
};

pub struct DiscordBotService<'a> {
    pub db: &'a DatabaseConnection,
    pub oauth_client: &'a OAuth2Client,
}

impl<'a> DiscordBotService<'a> {
    pub fn new(db: &'a DatabaseConnection, oauth_client: &'a OAuth2Client) -> Self {
        Self { db, oauth_client }
    }

    /// Generates a URL to add Discord bot to server
    pub async fn bot_url(&self, user_id: i32) -> Result<(Url, CsrfToken), AppError> {
        let user = UserRepository::new(&self.db);

        let Some(user) = user.find_by_id(user_id).await? else {
            return Err(AuthError::UserNotInDatabase(user_id).into());
        };

        if !user.admin {
            return Err(AuthError::AccessDenied(
                user_id,
                "User attempted to add bot to Discord server but doesn't have required admin permissions".to_string()
            ).into());
        }

        let (mut authorize_url, csrf_token) = self
            .oauth_client
            .authorize_url(|| CsrfToken::new_random())
            // Request scope to add bot and slash commands
            .add_scope(Scope::new("bot".to_string()))
            .add_scope(Scope::new("applications.commands".to_string()))
            .url();

        let permissions =
            Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES | Permissions::MENTION_EVERYONE;

        authorize_url
            .query_pairs_mut()
            .append_pair("permissions", &permissions.bits().to_string());

        Ok((authorize_url, csrf_token))
    }
}

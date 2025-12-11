use oauth2::{
    basic::BasicTokenType, AuthorizationCode, EmptyExtraTokenFields, StandardTokenResponse,
    TokenResponse,
};
use serenity::all::User as DiscordUser;

use crate::server::{
    error::{auth::AuthError, AppError},
    service::auth::DiscordAuthService,
};

impl DiscordAuthService {
    pub async fn callback(&self, authorization_code: String) -> Result<DiscordUser, AppError> {
        let auth_code = AuthorizationCode::new(authorization_code);

        let token = self
            .oauth_client
            .exchange_code(auth_code)
            .request_async(&self.http_client)
            .await
            .map_err(AuthError::from)?;

        let user = self.fetch_discord_user(&token).await?;

        Ok(user)
    }

    /// Retrieves a Discord user's information using provided access token
    async fn fetch_discord_user(
        &self,
        token: &StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    ) -> Result<DiscordUser, AppError> {
        let access_token = token.access_token().secret();

        let user_info = self
            .http_client
            .get("https://discord.com/api/users/@me")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<DiscordUser>()
            .await?;

        Ok(user_info)
    }
}

//! OAuth2 login with Discord

use crate::server::state::OAuth2Client;

pub mod callback;
pub mod login;

pub struct DiscordAuthService {
    pub http_client: reqwest::Client,
    pub oauth_client: OAuth2Client,
}

impl DiscordAuthService {
    pub fn new(http_client: reqwest::Client, oauth_client: OAuth2Client) -> Self {
        Self {
            http_client,
            oauth_client,
        }
    }
}

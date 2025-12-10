use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use tower_sessions::Session;

/// Session key for CSRF token
static SESSION_OAUTH_CSRF_TOKEN: &str = "oauth:csrf_token";

use crate::server::{error::AppError, service::oauth::DiscordAuthService, state::AppState};

pub async fn login(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let login_service = DiscordAuthService::new(state.oauth_client);

    let (url, csrf_token) = login_service.login_url();

    // Store CSRF token in session for verification during callback
    session
        .insert(SESSION_OAUTH_CSRF_TOKEN, csrf_token.secret())
        .await?;

    Ok(Redirect::temporary(&url.to_string()))
}

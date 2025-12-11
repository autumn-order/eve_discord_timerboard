use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use serde::Deserialize;
use tower_sessions::Session;

/// Session key for CSRF token
static SESSION_OAUTH_CSRF_TOKEN: &str = "oauth:csrf_token";
/// Session key for admin code validation
static SESSION_ADMIN_CODE_VALIDATED: &str = "admin:code_validated";

use crate::server::{
    data::discord::user::DiscordUserRepository,
    error::{auth::AuthError, AppError},
    service::auth::DiscordAuthService,
    state::AppState,
};

/// Query parameters for the login endpoint.
///
/// # Fields
/// - `admin_code` - Code to set the user as admin on login
#[derive(Deserialize)]
pub struct LoginParams {
    /// Code will be validated, setting the user as admin if successful
    pub admin_code: Option<String>,
}

/// Query parameters for the OAuth callback endpoint.
///
/// # Fields
/// - `state` - CSRF protection token that must match the value stored in the session
/// - `code` - Authorization code used to exchange for access tokens
#[derive(Deserialize)]
pub struct CallbackParams {
    /// CSRF state token to be validated against the session value.
    pub state: String,
    /// Authorization code from Discord SSO for token exchange.
    pub code: String,
}

pub async fn login(
    State(state): State<AppState>,
    session: Session,
    params: Query<LoginParams>,
) -> Result<impl IntoResponse, AppError> {
    let auth_service = DiscordAuthService::new(state.http_client, state.oauth_client);
    let admin_code = &params.0.admin_code;

    // Validate admin code if provided
    if let Some(code) = admin_code {
        let is_valid = state.admin_code_service.validate_and_consume(code).await;

        if !is_valid {
            return Err(AppError::AuthErr(AuthError::AdminCodeValidationFailed));
        }

        // Store admin code validation success in session
        session.insert(SESSION_ADMIN_CODE_VALIDATED, true).await?;
    }

    let (url, csrf_token) = auth_service.login_url();

    // Store CSRF token in session for verification during callback
    session
        .insert(SESSION_OAUTH_CSRF_TOKEN, csrf_token.secret())
        .await?;

    Ok(Redirect::temporary(&url.to_string()))
}

pub async fn callback(
    State(state): State<AppState>,
    session: Session,
    params: Query<CallbackParams>,
) -> Result<impl IntoResponse, AppError> {
    let auth_service = DiscordAuthService::new(state.http_client, state.oauth_client);
    let discord_user_repo = DiscordUserRepository::new(&state.db);

    validate_csrf(&session, &params.0.state).await?;

    // Check if admin code was validated in the login flow
    let is_admin: bool = session
        .remove(SESSION_ADMIN_CODE_VALIDATED)
        .await?
        .unwrap_or(false);

    let user = auth_service.callback(params.0.code).await?;
    let _new_user = discord_user_repo.upsert(user.clone(), is_admin).await?;

    Ok((StatusCode::OK, Json(user)))
}

async fn validate_csrf(session: &Session, csrf_state: &str) -> Result<(), AppError> {
    let stored_state: Option<String> = session.remove(SESSION_OAUTH_CSRF_TOKEN).await?;

    if let Some(state) = stored_state {
        if state == csrf_state {
            return Ok(());
        }
    }

    Err(AppError::AuthErr(AuthError::CsrfValidationFailed))
}

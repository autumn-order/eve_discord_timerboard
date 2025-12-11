use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use tower_sessions::Session;

use crate::server::{
    controller::auth::{SESSION_AUTH_CSRF_TOKEN, SESSION_AUTH_USER_ID},
    error::{auth::AuthError, AppError},
    service::admin::bot::DiscordBotService,
    state::AppState,
};

pub async fn add_bot(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let bot_service = DiscordBotService::new(&state.db, &state.oauth_client);

    let Some(user_id) = session.get(SESSION_AUTH_USER_ID).await? else {
        return Err(AuthError::UserNotInSession.into());
    };

    let (url, csrf_token) = bot_service.bot_url(user_id).await?;

    session
        .insert(SESSION_AUTH_CSRF_TOKEN, csrf_token.secret())
        .await?;

    Ok(Redirect::temporary(url.as_str()))
}

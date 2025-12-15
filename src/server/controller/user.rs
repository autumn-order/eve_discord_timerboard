use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tower_sessions::Session;

use crate::server::{
    error::AppError, middleware::auth::AuthGuard, service::user::UserService, state::AppState,
};

/// GET /api/user/guilds - Get all guilds available to the current user
///
/// Returns a list of all Discord guilds (timerboards) that the authenticated user
/// has access to. If the user is an admin, returns ALL guilds in the system.
/// If the user is not an admin, returns only guilds the user is a member of.
///
/// # Authentication
/// Requires user to be logged in (no admin permission required)
///
/// # Returns
/// - `200 OK`: JSON array of DiscordGuildDto (all guilds for admins, user's guilds otherwise)
/// - `500 Internal Server Error`: Database or parsing error
pub async fn get_user_guilds(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let auth_guard = AuthGuard::new(&state.db, &session);
    let user = auth_guard.require(&[]).await?;

    let user_id = user
        .discord_id
        .parse::<u64>()
        .map_err(|e| AppError::InternalError(format!("Failed to parse user discord_id: {}", e)))?;

    let user_service = UserService::new(&state.db);
    let guilds = user_service.get_user_guilds(user_id).await?;

    Ok((StatusCode::OK, Json(guilds)))
}

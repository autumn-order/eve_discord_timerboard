use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use dioxus_logger::tracing;
use serde::Deserialize;
use tower_sessions::Session;

use crate::server::{
    error::AppError,
    middleware::auth::{AuthGuard, Permission},
    service::discord::{DiscordGuildChannelService, DiscordGuildRoleService, DiscordGuildService},
    state::AppState,
};

#[derive(Deserialize)]
pub struct PaginationParams {
    #[serde(default)]
    pub page: u64,
    #[serde(default = "default_entries")]
    pub entries: u64,
}

fn default_entries() -> u64 {
    10
}

pub async fn get_all_discord_guilds(
    State(state): State<AppState>,
    session: Session,
) -> Result<impl IntoResponse, AppError> {
    let _ = AuthGuard::new(&state.db, &session)
        .require(&[Permission::Admin])
        .await?;

    let guild_service = DiscordGuildService::new(&state.db);

    let guilds = guild_service.get_all().await?;

    Ok((StatusCode::OK, Json(guilds)))
}

pub async fn get_discord_guild_by_id(
    State(state): State<AppState>,
    session: Session,
    Path(guild_id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let _ = AuthGuard::new(&state.db, &session)
        .require(&[Permission::Admin])
        .await?;

    tracing::info!("Guild ID: {}", guild_id);

    let guild_service = DiscordGuildService::new(&state.db);

    let Some(guild) = guild_service.get_by_guild_id(guild_id).await? else {
        return Err(AppError::NotFound(format!(
            "Guild with ID {} not found",
            guild_id
        )));
    };

    Ok((StatusCode::OK, Json(guild)))
}

/// GET /api/timerboard/{guild_id}/discord/roles
/// Get paginated roles for a guild
pub async fn get_discord_guild_roles(
    State(state): State<AppState>,
    session: Session,
    Path(guild_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse, AppError> {
    let _ = AuthGuard::new(&state.db, &session)
        .require(&[Permission::Admin])
        .await?;

    let service = DiscordGuildRoleService::new(&state.db);

    let roles = service
        .get_paginated(guild_id as u64, params.page, params.entries)
        .await?;

    Ok((StatusCode::OK, Json(roles)))
}

/// GET /api/timerboard/{guild_id}/discord/channels
/// Get paginated channels for a guild
pub async fn get_discord_guild_channels(
    State(state): State<AppState>,
    session: Session,
    Path(guild_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse, AppError> {
    let _ = AuthGuard::new(&state.db, &session)
        .require(&[Permission::Admin])
        .await?;

    let service = DiscordGuildChannelService::new(&state.db);

    let channels = service
        .get_paginated(guild_id as u64, params.page, params.entries)
        .await?;

    Ok((StatusCode::OK, Json(channels)))
}

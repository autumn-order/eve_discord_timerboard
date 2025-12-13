use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use tower_sessions::Session;

use crate::{
    model::fleet::{CreateFleetCategoryDto, FleetCategoryDto, UpdateFleetCategoryDto},
    server::{
        error::AppError,
        middleware::auth::{AuthGuard, Permission},
        service::fleet::FleetCategoryService,
        state::AppState,
    },
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

/// POST /api/timerboard/{guild_id}/fleet/category
/// Create a new fleet category
pub async fn create_fleet_category(
    State(state): State<AppState>,
    session: Session,
    Path(guild_id): Path<i64>,
    Json(payload): Json<CreateFleetCategoryDto>,
) -> Result<impl IntoResponse, AppError> {
    let _ = AuthGuard::new(&state.db, &session)
        .require(&[Permission::Admin])
        .await?;

    let service = FleetCategoryService::new(&state.db);

    let category = service
        .create(
            guild_id,
            payload.ping_format_id,
            payload.name,
            payload.ping_lead_time,
            payload.ping_reminder,
            payload.max_pre_ping,
        )
        .await?;

    Ok((StatusCode::CREATED, Json(category)))
}

/// GET /api/timerboard/{guild_id}/fleet/category
/// Get paginated fleet categories for a guild
pub async fn get_fleet_categories(
    State(state): State<AppState>,
    session: Session,
    Path(guild_id): Path<i64>,
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse, AppError> {
    let _ = AuthGuard::new(&state.db, &session)
        .require(&[Permission::Admin])
        .await?;

    let service = FleetCategoryService::new(&state.db);

    let categories = service
        .get_paginated(guild_id, params.page, params.entries)
        .await?;

    Ok((StatusCode::OK, Json(categories)))
}

/// PUT /api/timerboard/{guild_id}/fleet/category/{fleet_id}
/// Update a fleet category's name
pub async fn update_fleet_category(
    State(state): State<AppState>,
    session: Session,
    Path((guild_id, fleet_id)): Path<(i64, i32)>,
    Json(payload): Json<UpdateFleetCategoryDto>,
) -> Result<impl IntoResponse, AppError> {
    let _ = AuthGuard::new(&state.db, &session)
        .require(&[Permission::Admin])
        .await?;

    let service = FleetCategoryService::new(&state.db);

    let category = service
        .update(
            fleet_id,
            guild_id,
            payload.ping_format_id,
            payload.name,
            payload.ping_lead_time,
            payload.ping_reminder,
            payload.max_pre_ping,
        )
        .await?;

    match category {
        Some(cat) => Ok((StatusCode::OK, Json(cat))),
        None => Ok((
            StatusCode::NOT_FOUND,
            Json(FleetCategoryDto {
                id: 0,
                guild_id: 0,
                ping_format_id: 0,
                ping_format_name: String::new(),
                name: String::new(),
                ping_lead_time: None,
                ping_reminder: None,
                max_pre_ping: None,
            }),
        )),
    }
}

/// GET /api/timerboard/{guild_id}/fleet/category/by-ping-format/{ping_format_id}
/// Get fleet categories by ping format ID
pub async fn get_fleet_categories_by_ping_format(
    State(state): State<AppState>,
    session: Session,
    Path((_guild_id, ping_format_id)): Path<(i64, i32)>,
) -> Result<impl IntoResponse, AppError> {
    let _ = AuthGuard::new(&state.db, &session)
        .require(&[Permission::Admin])
        .await?;

    let service = FleetCategoryService::new(&state.db);

    let categories = service.get_by_ping_format_id(ping_format_id).await?;

    Ok((StatusCode::OK, Json(categories)))
}

/// DELETE /api/timerboard/{guild_id}/fleet/category/{fleet_id}
/// Delete a fleet category
pub async fn delete_fleet_category(
    State(state): State<AppState>,
    session: Session,
    Path((guild_id, fleet_id)): Path<(i64, i32)>,
) -> Result<impl IntoResponse, AppError> {
    let _ = AuthGuard::new(&state.db, &session)
        .require(&[Permission::Admin])
        .await?;

    let service = FleetCategoryService::new(&state.db);

    let deleted = service.delete(fleet_id, guild_id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}

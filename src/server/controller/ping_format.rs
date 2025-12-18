use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use tower_sessions::Session;

use crate::{
    model::ping_format::{CreatePingFormatDto, PingFormatDto, UpdatePingFormatDto},
    server::{
        error::AppError,
        middleware::auth::{AuthGuard, Permission},
        model::ping_format::{
            CreatePingFormatWithFieldsParam, DeletePingFormatParam, GetPaginatedPingFormatsParam,
            UpdatePingFormatWithFieldsParam,
        },
        service::ping_format::PingFormatService,
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

/// POST /api/timerboard/{guild_id}/ping/format
/// Create a new ping format
pub async fn create_ping_format(
    State(state): State<AppState>,
    session: Session,
    Path(guild_id): Path<u64>,
    Json(payload): Json<CreatePingFormatDto>,
) -> Result<impl IntoResponse, AppError> {
    let _ = AuthGuard::new(&state.db, &session)
        .require(&[Permission::Admin])
        .await?;

    let service = PingFormatService::new(&state.db);

    let fields: Vec<(String, i32, Option<String>)> = payload
        .fields
        .into_iter()
        .map(|f| (f.name, f.priority, f.default_value))
        .collect();

    let param = CreatePingFormatWithFieldsParam {
        guild_id,
        name: payload.name,
        fields,
    };

    let ping_format = service.create(param).await?;
    let dto = ping_format.into_dto().map_err(|e| {
        AppError::InternalError(format!("Failed to convert ping format to DTO: {}", e))
    })?;

    Ok((StatusCode::CREATED, Json(dto)))
}

/// GET /api/timerboard/{guild_id}/ping/format
/// Get paginated ping formats for a guild (with all fields per format)
pub async fn get_ping_formats(
    State(state): State<AppState>,
    session: Session,
    Path(guild_id): Path<u64>,
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse, AppError> {
    let _ = AuthGuard::new(&state.db, &session)
        .require(&[Permission::Admin])
        .await?;

    let service = PingFormatService::new(&state.db);

    let param = GetPaginatedPingFormatsParam {
        guild_id,
        page: params.page,
        per_page: params.entries,
    };

    let ping_formats = service.get_paginated(param).await?;
    let dto = ping_formats.into_dto().map_err(|e| {
        AppError::InternalError(format!("Failed to convert ping formats to DTO: {}", e))
    })?;

    Ok((StatusCode::OK, Json(dto)))
}

/// PUT /api/timerboard/{guild_id}/ping/format/{format_id}
/// Update a ping format's name and fields
pub async fn update_ping_format(
    State(state): State<AppState>,
    session: Session,
    Path((guild_id, format_id)): Path<(u64, i32)>,
    Json(payload): Json<UpdatePingFormatDto>,
) -> Result<impl IntoResponse, AppError> {
    let _ = AuthGuard::new(&state.db, &session)
        .require(&[Permission::Admin])
        .await?;

    let service = PingFormatService::new(&state.db);

    let fields: Vec<(Option<i32>, String, i32, Option<String>)> = payload
        .fields
        .into_iter()
        .map(|f| (f.id, f.name, f.priority, f.default_value))
        .collect();

    let param = UpdatePingFormatWithFieldsParam {
        id: format_id,
        guild_id,
        name: payload.name,
        fields,
    };

    let ping_format = service.update(param).await?;

    match ping_format {
        Some(pf) => {
            let dto = pf.into_dto().map_err(|e| {
                AppError::InternalError(format!("Failed to convert ping format to DTO: {}", e))
            })?;
            Ok((StatusCode::OK, Json(dto)))
        }
        None => Ok((
            StatusCode::NOT_FOUND,
            Json(PingFormatDto {
                id: 0,
                guild_id: 0,
                name: String::new(),
                fields: Vec::new(),
                fleet_category_count: 0,
                fleet_category_names: Vec::new(),
            }),
        )),
    }
}

/// DELETE /api/timerboard/{guild_id}/ping/format/{format_id}
/// Delete a ping format
pub async fn delete_ping_format(
    State(state): State<AppState>,
    session: Session,
    Path((guild_id, format_id)): Path<(u64, i32)>,
) -> Result<impl IntoResponse, AppError> {
    let _ = AuthGuard::new(&state.db, &session)
        .require(&[Permission::Admin])
        .await?;

    let service = PingFormatService::new(&state.db);

    let param = DeletePingFormatParam {
        id: format_id,
        guild_id,
    };

    let deleted = service.delete(param).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}

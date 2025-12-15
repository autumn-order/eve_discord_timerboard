use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use tower_sessions::Session;

use crate::{
    model::{
        category::{FleetCategoryDetailsDto, PingFormatFieldDto},
        discord::DiscordGuildMemberDto,
    },
    server::{
        data::{
            category::FleetCategoryRepository, discord::user_guild::UserDiscordGuildRepository,
        },
        error::AppError,
        middleware::auth::{AuthGuard, Permission},
        state::AppState,
    },
};

/// GET /api/guilds/{guild_id}/categories/{category_id}/details
/// Get category details including ping format fields for fleet creation
pub async fn get_category_details(
    State(state): State<AppState>,
    session: Session,
    Path((guild_id, category_id)): Path<(u64, i32)>,
) -> Result<impl IntoResponse, AppError> {
    let user = AuthGuard::new(&state.db, &session)
        .require(&[Permission::CategoryView(guild_id, category_id)])
        .await?;

    let category_repo = FleetCategoryRepository::new(&state.db);
    let category_with_relations = category_repo
        .get_category_details(category_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

    // Get ping format fields
    let fields = entity::prelude::PingFormatField::find()
        .filter(
            entity::ping_format_field::Column::PingFormatId
                .eq(category_with_relations.category.ping_format_id),
        )
        .order_by_asc(entity::ping_format_field::Column::Priority)
        .all(&state.db)
        .await?
        .into_iter()
        .map(|f| PingFormatFieldDto {
            id: f.id,
            name: f.name,
            priority: f.priority,
        })
        .collect();

    // Build the response DTO
    let dto = FleetCategoryDetailsDto {
        id: category_with_relations.category.id,
        guild_id,
        ping_format_id: category_with_relations.category.ping_format_id,
        ping_format_name: category_with_relations
            .ping_format
            .map(|pf| pf.name)
            .unwrap_or_default(),
        name: category_with_relations.category.name.clone(),
        ping_lead_time: category_with_relations
            .category
            .ping_cooldown
            .map(|seconds| chrono::Duration::seconds(seconds as i64)),
        ping_reminder: category_with_relations
            .category
            .ping_reminder
            .map(|seconds| chrono::Duration::seconds(seconds as i64)),
        max_pre_ping: category_with_relations
            .category
            .max_pre_ping
            .map(|seconds| chrono::Duration::seconds(seconds as i64)),
        access_roles: category_with_relations
            .access_roles
            .into_iter()
            .filter_map(|(access_role, role_model)| {
                role_model.map(|role| crate::model::category::FleetCategoryAccessRoleDto {
                    role_id: role.role_id.parse().unwrap_or(0),
                    role_name: role.name,
                    role_color: role.color,
                    position: role.position,
                    can_view: access_role.can_view,
                    can_create: access_role.can_create,
                    can_manage: access_role.can_manage,
                })
            })
            .collect(),
        ping_roles: category_with_relations
            .ping_roles
            .into_iter()
            .filter_map(|(_ping_role, role_model)| {
                role_model.map(|role| crate::model::category::FleetCategoryPingRoleDto {
                    role_id: role.role_id.parse().unwrap_or(0),
                    role_name: role.name,
                    role_color: role.color,
                    position: role.position,
                })
            })
            .collect(),
        channels: category_with_relations
            .channels
            .into_iter()
            .filter_map(|(_cat_channel, channel_model)| {
                channel_model.map(|channel| crate::model::category::FleetCategoryChannelDto {
                    channel_id: channel.channel_id.parse().unwrap_or(0),
                    channel_name: channel.name,
                    position: channel.position,
                })
            })
            .collect(),
        fields,
    };

    Ok((StatusCode::OK, Json(dto)))
}

/// GET /api/guilds/{guild_id}/members
/// Get all members of a guild for FC selection
pub async fn get_guild_members(
    State(state): State<AppState>,
    session: Session,
    Path(guild_id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let _user = AuthGuard::new(&state.db, &session).require(&[]).await?;

    let user_guild_repo = UserDiscordGuildRepository::new(&state.db);
    let members = user_guild_repo.get_guild_members(guild_id).await?;

    let member_dtos: Vec<DiscordGuildMemberDto> = members
        .into_iter()
        .map(|user| DiscordGuildMemberDto {
            user_id: user.discord_id.parse().unwrap_or(0),
            username: user.name.clone(),
            display_name: user.name.clone(),
            avatar_hash: None,
        })
        .collect();

    Ok((StatusCode::OK, Json(member_dtos)))
}

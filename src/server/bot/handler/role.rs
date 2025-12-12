use dioxus_logger::tracing;
use sea_orm::DatabaseConnection;
use serenity::all::{Context, GuildId, Role, RoleId};

use crate::server::data::discord::DiscordGuildRoleRepository;

/// Handles the guild_role_create event when a role is created in a guild
pub async fn handle_guild_role_create(db: &DatabaseConnection, _ctx: Context, new: Role) {
    let guild_id = new.guild_id.get();
    let role_repo = DiscordGuildRoleRepository::new(db);

    if let Err(e) = role_repo.upsert(guild_id, &new).await {
        tracing::error!("Failed to upsert new role: {:?}", e);
    } else {
        tracing::info!("Created role {} in guild {}", new.name, guild_id);
    }
}

/// Handles the guild_role_update event when a role is updated in a guild
pub async fn handle_guild_role_update(
    db: &DatabaseConnection,
    _ctx: Context,
    _old: Option<Role>,
    new: Role,
) {
    let guild_id = new.guild_id.get();
    let role_repo = DiscordGuildRoleRepository::new(db);

    if let Err(e) = role_repo.upsert(guild_id, &new).await {
        tracing::error!("Failed to upsert updated role: {:?}", e);
    } else {
        tracing::info!("Updated role {} in guild {}", new.name, guild_id);
    }
}

/// Handles the guild_role_delete event when a role is deleted from a guild
pub async fn handle_guild_role_delete(
    db: &DatabaseConnection,
    _ctx: Context,
    guild_id: GuildId,
    removed_role_id: RoleId,
    _removed_role_data_if_in_cache: Option<Role>,
) {
    let role_repo = DiscordGuildRoleRepository::new(db);

    if let Err(e) = role_repo.delete(removed_role_id.get()).await {
        tracing::error!("Failed to delete role: {:?}", e);
    } else {
        tracing::info!("Deleted role {} from guild {}", removed_role_id, guild_id);
    }
}

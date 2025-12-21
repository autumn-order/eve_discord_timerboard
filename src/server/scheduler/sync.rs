use chrono::{Duration, Utc};
use dioxus_logger::tracing;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};
use serenity::http::Http;
use std::sync::Arc;

use crate::server::{
    data::discord::DiscordGuildRepository,
    error::AppError,
    model::discord::guild::UpsertGuildParam,
    service::discord::{
        DiscordGuildChannelService, DiscordGuildMemberService, DiscordGuildRoleService,
    },
};

static GUILD_SYNC_THRESHOLD: usize = 30;
static GUILD_SYNC_CHECK_INTERVAL: usize = 5;

pub async fn process_guild_sync(
    db: &DatabaseConnection,
    discord_http: Arc<Http>,
) -> Result<(), AppError> {
    let guild_count = entity::prelude::DiscordGuild::find().count(db).await?;

    // Space out sync evenly across the threshold window
    let limit = guild_count * (GUILD_SYNC_CHECK_INTERVAL / GUILD_SYNC_THRESHOLD) as u64;

    // Retrieve guilds that are in need of a full sync
    let sync_threshold = Utc::now() - Duration::minutes(GUILD_SYNC_THRESHOLD as i64);
    let guilds: Vec<String> = entity::prelude::DiscordGuild::find()
        .filter(entity::discord_guild::Column::LastSyncAt.lt(sync_threshold))
        .select_only()
        .column(entity::discord_guild::Column::GuildId)
        .limit(limit)
        .into_tuple()
        .all(db)
        .await?;

    // Filter out and log an error for any guild IDs that fail to parse to u64
    let guild_ids: Vec<u64> = guilds
        .iter()
        .filter_map(|id| match id.parse::<u64>() {
            Ok(parsed_id) => Some(parsed_id),
            Err(e) => {
                tracing::error!("Failed to parse guild_id '{}': {}", id, e);
                None
            }
        })
        .collect();

    for guild_id in guild_ids {
        // Fetch guild data from Discord API
        let guild_id_obj = serenity::model::id::GuildId::new(guild_id);

        let guild = match discord_http.get_guild(guild_id_obj).await {
            Ok(guild) => guild,
            Err(e) => {
                tracing::error!("Failed to fetch guild information {}: {}", guild_id, e);
                continue;
            }
        };

        let guild_repo = DiscordGuildRepository::new(db);

        let guild_name = guild.name.clone();

        // Always upsert basic guild metadata (name, icon, member count)
        let param = UpsertGuildParam::from_partial_guild(&guild);
        if let Err(e) = guild_repo.upsert(param).await {
            tracing::error!(
                "Failed to upsert guild {} ({}): {:?}",
                guild_id,
                guild_name,
                e
            );
            continue;
        }

        tracing::trace!("Syncing guild {} ({})", guild.name, guild_id);

        // Fetch roles and channels
        let guild_roles = match discord_http.get_guild_roles(guild_id_obj).await {
            Ok(roles) => roles,
            Err(e) => {
                tracing::error!("Failed to fetch roles for guild {}: {}", guild_id, e);
                continue;
            }
        };

        // Sync all roles in the guild
        let role_service = DiscordGuildRoleService::new(db);

        if let Err(e) = role_service.update_roles(guild_id, &guild_roles).await {
            tracing::error!("Failed to update guild {} roles: {:?}", guild_id, e);
        }

        let guild_channels = match discord_http.get_channels(guild_id_obj).await {
            Ok(channels) => channels,
            Err(e) => {
                tracing::error!("Failed to fetch channels for guild {}: {}", guild_id, e);
                continue;
            }
        };

        // Sync all text channels in the guild
        let channel_service = DiscordGuildChannelService::new(db);

        if let Err(e) = channel_service
            .update_channels(guild_id, &guild_channels)
            .await
        {
            tracing::error!("Failed to update guild {} channels: {:?}", guild_id, e);
        }

        let member_service = DiscordGuildMemberService::new(db, discord_http.clone());
        if let Err(e) = member_service.sync_guild_members(guild_id).await {
            tracing::error!("Failed to sync guild {} members: {:?}", guild_id, e);
        }

        // Update last sync timestamp after successful sync
        if let Err(e) = guild_repo.update_last_sync(guild_id).await {
            tracing::error!(
                "Failed to update guild {} last sync timestamp: {:?}",
                guild_id,
                e
            );
        } else {
            tracing::debug!("Successfully completed full sync for guild {}", guild_id);
        }
    }

    Ok(())
}

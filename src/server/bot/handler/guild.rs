use dioxus_logger::tracing;
use sea_orm::DatabaseConnection;
use serenity::all::{Context, Guild};

use crate::server::data::discord::DiscordGuildRepository;
use crate::server::service::discord::{
    DiscordGuildChannelService, DiscordGuildRoleService, UserDiscordGuildRoleService,
    UserDiscordGuildService,
};

/// Handles the guild_create event when a guild becomes available or the bot joins a new guild
///
/// This event fires on bot startup for each guild the bot is in, and when the bot joins a new guild.
/// It syncs all guild data (metadata, roles, channels, members) to ensure the database is up-to-date.
pub async fn handle_guild_create(
    db: &DatabaseConnection,
    ctx: Context,
    guild: Guild,
    _is_new: Option<bool>,
) {
    let guild_id = guild.id.get();
    let guild_roles = guild.roles.clone();
    let guild_channels = guild.channels.clone();
    let cached_members = guild.members.clone();

    tracing::debug!(
        "Guild create event: {} ({}) - member_count: {}, cached_members: {}",
        guild.name,
        guild_id,
        guild.member_count,
        cached_members.len()
    );

    let guild_repo = DiscordGuildRepository::new(db);
    let user_guild_service = UserDiscordGuildService::new(db);

    if let Err(e) = guild_repo.upsert(guild).await {
        tracing::error!("Failed to upsert guild: {:?}", e);
        return;
    }

    let role_service = DiscordGuildRoleService::new(db);

    if let Err(e) = role_service.update_roles(guild_id, &guild_roles).await {
        tracing::error!("Failed to update guild roles: {:?}", e);
    }

    let channel_service = DiscordGuildChannelService::new(db);

    if let Err(e) = channel_service
        .update_channels(guild_id, &guild_channels)
        .await
    {
        tracing::error!("Failed to update guild channels: {:?}", e);
    }

    // Fetch members from Discord API since guild.members may not be populated
    // This requires the GUILD_MEMBERS privileged intent
    let members = match ctx
        .http
        .get_guild_members(guild_id.into(), None, None)
        .await
    {
        Ok(members) => {
            tracing::debug!(
                "Fetched {} members from Discord API for guild {}",
                members.len(),
                guild_id
            );
            members
        }
        Err(e) => {
            tracing::error!("Failed to fetch guild members from API: {:?}", e);
            // Fallback to cached members if API call fails
            cached_members.values().cloned().collect()
        }
    };

    let member_ids: Vec<u64> = members.iter().map(|m| m.user.id.get()).collect();

    // Sync guild members to catch any missed join/leave events while bot was offline
    if let Err(e) = user_guild_service
        .sync_guild_members(guild_id, &member_ids)
        .await
    {
        tracing::error!("Failed to sync guild members: {:?}", e);
    }

    // Sync role memberships for logged-in users
    let user_role_service = UserDiscordGuildRoleService::new(db);
    if let Err(e) = user_role_service
        .sync_guild_member_roles(guild_id, &members)
        .await
    {
        tracing::error!("Failed to sync guild member roles: {:?}", e);
    }
}

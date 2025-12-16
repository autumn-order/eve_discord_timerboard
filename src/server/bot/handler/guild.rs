use dioxus_logger::tracing;
use sea_orm::DatabaseConnection;
use serenity::all::{Context, Guild};

use crate::server::data::discord::DiscordGuildRepository;
use crate::server::service::discord::{
    DiscordGuildChannelService, DiscordGuildMemberService, DiscordGuildRoleService,
    UserDiscordGuildRoleService,
};

/// Handles the guild_create event when a guild becomes available or the bot joins a new guild
///
/// This event fires on bot startup for each guild the bot is in, and when the bot joins a new guild.
/// It syncs all guild data (metadata, roles, channels, members) to ensure the database is up-to-date.
/// To prevent excessive syncs on frequent bot restarts, a 30-minute backoff is enforced.
pub async fn handle_guild_create(
    db: &DatabaseConnection,
    ctx: Context,
    guild: Guild,
    _is_new: Option<bool>,
) {
    let guild_id = guild.id.get();
    let guild_roles = guild.roles.clone();
    let guild_channels = guild.channels.clone();

    tracing::debug!(
        "Guild create event: {} ({}) - member_count: {}",
        guild.name,
        guild_id,
        guild.member_count,
    );

    let guild_repo = DiscordGuildRepository::new(db);

    // Always upsert basic guild metadata (name, icon)
    if let Err(e) = guild_repo.upsert(guild).await {
        tracing::error!("Failed to upsert guild: {:?}", e);
        return;
    }

    // Check if a full sync is needed (30-minute backoff)
    let needs_sync = match guild_repo.needs_sync(guild_id).await {
        Ok(needs) => needs,
        Err(e) => {
            tracing::error!("Failed to check if guild needs sync: {:?}", e);
            return;
        }
    };

    if !needs_sync {
        tracing::debug!(
            "Skipping full sync for guild {} (synced within last 30 minutes)",
            guild_id
        );
        return;
    }

    tracing::info!("Performing full sync for guild {}", guild_id);

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

    // Fetch ALL members from Discord API with pagination
    // This requires the GUILD_MEMBERS privileged intent
    let mut all_members = Vec::new();
    let mut after: Option<u64> = None;

    loop {
        match ctx
            .http
            .get_guild_members(guild_id.into(), Some(1000), after)
            .await
        {
            Ok(members) => {
                if members.is_empty() {
                    break;
                }

                tracing::debug!(
                    "Fetched {} members from Discord API for guild {} (total so far: {})",
                    members.len(),
                    guild_id,
                    all_members.len() + members.len()
                );

                // Set up pagination for next iteration
                after = members.last().map(|m| m.user.id.get());

                // Add to our collection
                all_members.extend(members);

                // If we got less than 1000, we've reached the end
                if all_members.len() < 1000 {
                    break;
                }
            }
            Err(e) => {
                tracing::error!("Failed to fetch guild members from API: {:?}", e);
                break;
            }
        }
    }

    tracing::info!(
        "Fetched total of {} members for guild {}",
        all_members.len(),
        guild_id
    );

    // Convert to the format needed for sync: (user_id, username, nickname)
    let member_data: Vec<(u64, String, Option<String>)> = all_members
        .iter()
        .map(|m| (m.user.id.get(), m.user.name.clone(), m.nick.clone()))
        .collect();

    // Sync ALL guild members (not just logged-in users)
    let member_service = DiscordGuildMemberService::new(db);
    if let Err(e) = member_service
        .sync_guild_members(guild_id, &member_data)
        .await
    {
        tracing::error!("Failed to sync guild members: {:?}", e);
    }

    // Sync role memberships for logged-in users only
    let user_role_service = UserDiscordGuildRoleService::new(db);
    if let Err(e) = user_role_service
        .sync_guild_member_roles(guild_id, &all_members)
        .await
    {
        tracing::error!("Failed to sync guild member roles: {:?}", e);
    }

    // Update last sync timestamp after successful sync
    if let Err(e) = guild_repo.update_last_sync(guild_id).await {
        tracing::error!("Failed to update guild last sync timestamp: {:?}", e);
    } else {
        tracing::info!("Successfully synced guild {} data", guild_id);
    }
}

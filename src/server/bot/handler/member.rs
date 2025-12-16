use dioxus_logger::tracing;
use sea_orm::DatabaseConnection;
use serenity::all::{Context, GuildId, GuildMemberUpdateEvent, Member, User};

use crate::server::service::discord::{DiscordGuildMemberService, UserDiscordGuildRoleService};

/// Handles the guild_member_addition event when a member joins a guild
pub async fn handle_guild_member_addition(
    db: &DatabaseConnection,
    _ctx: Context,
    new_member: Member,
) {
    let user_id = new_member.user.id.get();
    let guild_id = new_member.guild_id.get();
    let username = new_member.user.name.clone();
    let nickname = new_member.nick.clone();

    tracing::info!(
        "Member {} ({}) joined guild {}",
        username,
        user_id,
        guild_id
    );

    // Add member to guild_member table (tracks ALL members)
    let member_service = DiscordGuildMemberService::new(db);
    if let Err(e) = member_service
        .upsert_member(user_id, guild_id, username, nickname)
        .await
    {
        tracing::error!("Failed to add guild member: {:?}", e);
        return;
    }

    // If this user has an application account, sync their roles
    let user_role_service = UserDiscordGuildRoleService::new(db);
    if let Err(e) = user_role_service
        .sync_user_roles(user_id, &new_member)
        .await
    {
        // This will fail silently if user doesn't have an app account - that's fine
        tracing::debug!(
            "Did not sync roles for user {} (likely not logged into app): {:?}",
            user_id,
            e
        );
    }
}

/// Handles the guild_member_removal event when a member leaves a guild
pub async fn handle_guild_member_removal(
    db: &DatabaseConnection,
    _ctx: Context,
    guild_id: GuildId,
    user: User,
    _member_data_if_available: Option<Member>,
) {
    let user_id = user.id.get();
    let guild_id = guild_id.get();

    tracing::info!("Member {} ({}) left guild {}", user.name, user_id, guild_id);

    // Remove member from guild_member table
    let member_service = DiscordGuildMemberService::new(db);
    if let Err(e) = member_service.remove_member(user_id, guild_id).await {
        tracing::error!("Failed to remove guild member: {:?}", e);
    }

    // Note: user_discord_guild_role records will be automatically deleted via CASCADE
    // when the user row is deleted (for logged-in users only)
}

/// Handles the guild_member_update event when a member is updated in a guild (roles, nickname, etc.)
pub async fn handle_guild_member_update(
    db: &DatabaseConnection,
    _ctx: Context,
    _old: Option<Member>,
    new: Option<Member>,
    _event: GuildMemberUpdateEvent,
) {
    let Some(member) = new else {
        return;
    };

    let user_id = member.user.id.get();
    let guild_id = member.guild_id.get();
    let username = member.user.name.clone();
    let nickname = member.nick.clone();

    tracing::debug!(
        "Member {} ({}) updated in guild {}",
        username,
        user_id,
        guild_id
    );

    // Update member in guild_member table (updates username/nickname)
    let member_service = DiscordGuildMemberService::new(db);
    if let Err(e) = member_service
        .upsert_member(user_id, guild_id, username, nickname)
        .await
    {
        tracing::error!("Failed to update guild member: {:?}", e);
        return;
    }

    // If this user has an application account, sync their roles
    let user_role_service = UserDiscordGuildRoleService::new(db);
    if let Err(e) = user_role_service.sync_user_roles(user_id, &member).await {
        tracing::debug!(
            "Did not sync roles for user {} (likely not logged into app): {:?}",
            user_id,
            e
        );
    }
}

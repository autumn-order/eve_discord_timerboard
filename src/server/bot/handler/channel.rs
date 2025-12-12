use dioxus_logger::tracing;
use sea_orm::DatabaseConnection;
use serenity::all::{ChannelType, Context, GuildChannel, Message};

use crate::server::data::discord::DiscordGuildChannelRepository;

/// Handles the channel_create event when a channel is created in a guild
pub async fn handle_channel_create(db: &DatabaseConnection, _ctx: Context, channel: GuildChannel) {
    let guild_id = channel.guild_id.get();
    let channel_repo = DiscordGuildChannelRepository::new(db);

    // Only track text channels
    if channel.kind != ChannelType::Text {
        return;
    }

    if let Err(e) = channel_repo.upsert(guild_id, &channel).await {
        tracing::error!("Failed to upsert new channel: {:?}", e);
    } else {
        tracing::info!("Created channel {} in guild {}", channel.name, guild_id);
    }
}

/// Handles the channel_update event when a channel is updated in a guild
pub async fn handle_channel_update(
    db: &DatabaseConnection,
    _ctx: Context,
    _old: Option<GuildChannel>,
    new: GuildChannel,
) {
    let channel = new;
    let guild_id = channel.guild_id.get();
    let channel_repo = DiscordGuildChannelRepository::new(db);

    // Only track text channels
    if channel.kind != ChannelType::Text {
        return;
    }

    if let Err(e) = channel_repo.upsert(guild_id, &channel).await {
        tracing::error!("Failed to upsert updated channel: {:?}", e);
    } else {
        tracing::info!("Updated channel {} in guild {}", channel.name, guild_id);
    }
}

/// Handles the channel_delete event when a channel is deleted from a guild
pub async fn handle_channel_delete(
    db: &DatabaseConnection,
    _ctx: Context,
    channel: GuildChannel,
    _messages: Option<Vec<Message>>,
) {
    let guild_id = channel.guild_id.get();
    let channel_id = channel.id.get();
    let channel_repo = DiscordGuildChannelRepository::new(db);

    if let Err(e) = channel_repo.delete(channel_id).await {
        tracing::error!("Failed to delete channel: {:?}", e);
    } else {
        tracing::info!("Deleted channel {} from guild {}", channel_id, guild_id);
    }
}

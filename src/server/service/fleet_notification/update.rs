//! Fleet notification update operations.
//!
//! This module provides functionality for updating existing fleet notifications.
//! It handles editing Discord messages with new fleet information.

use dioxus_logger::tracing;
use serenity::all::{ChannelId, EditMessage, MessageId};

use crate::server::{
    data::{
        category::FleetCategoryRepository, fleet_message::FleetMessageRepository,
        ping_format::field::PingFormatFieldRepository,
    },
    error::AppError,
    model::fleet::Fleet,
    util::parse::parse_u64_from_string,
};

use super::{builder, FleetNotificationService};

impl<'a> FleetNotificationService<'a> {
    /// Updates all existing fleet messages with new fleet information.
    ///
    /// Edits all Discord messages associated with the fleet to reflect updated details.
    /// Continues updating remaining messages even if individual updates fail. Uses blue
    /// embed color (0x3498db) for updates. Logs errors for failed updates but doesn't
    /// propagate them to allow partial success.
    ///
    /// # Arguments
    /// - `fleet` - Updated fleet domain model with current event details
    /// - `field_values` - Map of field_id to value for custom ping format fields
    ///
    /// # Returns
    /// - `Ok(())` - Successfully updated all messages (or no messages exist)
    /// - `Err(AppError::NotFound)` - Fleet category or ping format not found
    /// - `Err(AppError::InternalError)` - Invalid ID format or timestamp
    /// - `Err(AppError::Database)` - Database error retrieving messages or fields
    pub async fn update_fleet_messages(
        &self,
        fleet: &Fleet,
        field_values: &std::collections::HashMap<i32, String>,
    ) -> Result<(), AppError> {
        let message_repo = FleetMessageRepository::new(self.db);
        let ping_format_field_repo = PingFormatFieldRepository::new(self.db);
        let category_repo = FleetCategoryRepository::new(self.db);

        // Get all existing messages for this fleet
        let messages = message_repo.get_by_fleet_id(fleet.id).await?;

        if messages.is_empty() {
            tracing::debug!("No messages found for fleet {}, skipping update", fleet.id);
            return Ok(());
        }

        // Get category data
        let category_data = category_repo
            .find_by_id(fleet.category_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Fleet category not found".to_string()))?;

        // Get guild_id for fetching commander name
        let guild_id = parse_u64_from_string(category_data.category.guild_id)?;

        // Get ping format fields
        let ping_format = category_data
            .ping_format
            .ok_or_else(|| AppError::NotFound("Ping format not found".to_string()))?;

        let fields = ping_format_field_repo
            .get_by_ping_format_id(guild_id, ping_format.id)
            .await?;

        // Fetch commander name from Discord
        let commander_name =
            builder::get_commander_name(self.http.clone(), fleet, guild_id).await?;

        // Build updated embed (use blue color for updates)
        let embed = builder::build_fleet_embed(
            fleet,
            &fields,
            field_values,
            0x3498db,
            &commander_name,
            &self.app_url,
        )
        .await?;

        // Update each message
        for message in messages {
            let channel_id = ChannelId::new(message.channel_id);
            let msg_id = MessageId::new(message.message_id);

            let edit_builder = EditMessage::new().embed(embed.clone());

            match self
                .http
                .edit_message(channel_id, msg_id, &edit_builder, vec![])
                .await
            {
                Ok(_) => {
                    tracing::info!("Updated fleet message {} in channel {}", msg_id, channel_id);
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to update fleet message {} in channel {}: {}",
                        msg_id,
                        channel_id,
                        e
                    );
                    // Continue updating other messages even if one fails
                }
            }
        }

        Ok(())
    }
}

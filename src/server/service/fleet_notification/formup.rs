//! Fleet formup notification operations.
//!
//! This module provides functionality for posting formup (start) notifications for fleets at their start time.
//! Formup messages are posted as replies to the most recent existing messages (reminder or creation).

use dioxus_logger::tracing;
use serenity::all::{ChannelId, CreateMessage, MessageId, MessageReference};

use crate::server::{
    data::{
        category::FleetCategoryRepository, fleet_message::FleetMessageRepository,
        ping_format::field::PingFormatFieldRepository,
    },
    error::AppError,
    model::{fleet::Fleet, fleet_message::CreateFleetMessageParam},
    util::parse::parse_u64_from_string,
};

use super::{builder, FleetNotificationService};

impl<'a> FleetNotificationService<'a> {
    /// Posts fleet formup message as a reply to existing fleet messages.
    ///
    /// Creates formup notifications at fleet time to signal immediate gathering. Replies
    /// to the most recent existing message (reminder or creation) for each channel.
    /// Uses red embed color (0xe74c3c) to indicate urgency.
    ///
    /// # Arguments
    /// - `fleet` - Fleet domain model containing event details
    /// - `field_values` - Map of field_id to value for custom ping format fields
    ///
    /// # Returns
    /// - `Ok(())` - Successfully posted formup messages to all channels
    /// - `Err(AppError::NotFound)` - Fleet category or ping format not found
    /// - `Err(AppError::InternalError)` - Invalid ID format or timestamp
    /// - `Err(AppError::Database)` - Database error retrieving or storing messages
    pub async fn post_fleet_formup(
        &self,
        fleet: &Fleet,
        field_values: &std::collections::HashMap<i32, String>,
    ) -> Result<(), AppError> {
        let ping_format_field_repo = PingFormatFieldRepository::new(self.db);
        let category_repo = FleetCategoryRepository::new(self.db);
        let message_repo = FleetMessageRepository::new(self.db);

        // Get existing messages to reply to
        let existing_messages = message_repo.get_by_fleet_id(fleet.id).await?;

        if existing_messages.is_empty() {
            tracing::warn!(
                "No existing messages found for fleet {}, skipping formup",
                fleet.id
            );
            return Ok(());
        }

        // Get category with channels and ping roles
        let Some(category_data) = category_repo.find_by_id(fleet.category_id).await? else {
            return Err(AppError::NotFound("Fleet category not found".to_string()));
        };

        // Get guild_id for fetching commander name
        let guild_id = parse_u64_from_string(category_data.category.guild_id)?;

        // Get ping format fields for the category
        let Some(ping_format) = category_data.ping_format else {
            return Err(AppError::NotFound("Ping format not found".to_string()));
        };

        let fields = ping_format_field_repo
            .get_by_ping_format_id(guild_id, ping_format.id)
            .await?;

        // Fetch commander name from Discord
        let commander_name =
            builder::get_commander_name(self.http.clone(), fleet, guild_id).await?;

        // Build embed with red color for formup (urgency)
        let embed = builder::build_fleet_embed(
            fleet,
            &fields,
            field_values,
            0xe74c3c, // Red color for formup
            &commander_name,
            &self.app_url,
        )
        .await?;

        // Build title for formup
        let title = format!("**.:{} Forming Now:.**", category_data.category.name);

        // Build ping content with title
        let mut content = format!("{}\n\n", title);
        for (ping_role, _) in &category_data.ping_roles {
            let role_id = parse_u64_from_string(ping_role.role_id.clone())?;

            // @everyone role has the same ID as the guild - use @everyone instead of <@&guild_id>
            if role_id == guild_id {
                content.push_str("@everyone ");
            } else {
                content.push_str(&format!("<@&{}> ", role_id));
            }
        }

        // Post to all configured channels
        for (channel, _) in &category_data.channels {
            let channel_id_u64 = parse_u64_from_string(channel.channel_id.clone())?;
            let channel_id = ChannelId::new(channel_id_u64);

            // Find the most recent message for this channel (prefer reminder over creation)
            let reference_msg = existing_messages
                .iter()
                .filter(|m| m.channel_id == channel_id_u64)
                .max_by_key(|m| &m.created_at);

            let mut message = CreateMessage::new().content(&content).embed(embed.clone());

            // Reply to the most recent message if it exists
            if let Some(ref_msg) = reference_msg {
                message = message.reference_message(MessageReference::from((
                    channel_id,
                    MessageId::new(ref_msg.message_id),
                )));
            }

            match channel_id.send_message(&self.http, message).await {
                Ok(msg) => {
                    // Store message in database
                    message_repo
                        .create(CreateFleetMessageParam {
                            fleet_id: fleet.id,
                            channel_id: channel_id_u64,
                            message_id: msg.id.get(),
                            message_type: "formup".to_string(),
                        })
                        .await?;

                    tracing::info!(
                        "Posted fleet formup for fleet {} to channel {}",
                        fleet.id,
                        channel_id_u64
                    );
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to post fleet formup to channel {}: {}",
                        channel_id_u64,
                        e
                    );
                    // Continue posting to other channels even if one fails
                }
            }
        }

        Ok(())
    }
}

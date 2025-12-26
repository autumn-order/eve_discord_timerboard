//! Fleet creation notification operations.
//!
//! This module provides functionality for posting the initial creation notifications for new fleets.
//! Creation messages are posted to all configured channels with role pings and fleet details.

use dioxus_logger::tracing;
use serenity::all::{ChannelId, CreateMessage};

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
    /// Posts fleet creation message to all configured channels.
    ///
    /// Creates Discord messages with fleet details in all channels configured for the
    /// fleet's category. Only posts if the fleet is not hidden. Message IDs are stored
    /// in the database for later updates or cancellations. Uses blue embed color (0x3498db).
    ///
    /// # Arguments
    /// - `fleet` - Fleet domain model containing event details
    /// - `field_values` - Map of field_id to value for custom ping format fields
    ///
    /// # Returns
    /// - `Ok(())` - Successfully posted creation messages to all channels
    /// - `Err(AppError::NotFound)` - Fleet category or ping format not found
    /// - `Err(AppError::InternalError)` - Invalid ID format or timestamp
    /// - `Err(AppError::Database)` - Database error storing message records
    pub async fn post_fleet_creation(
        &self,
        fleet: &Fleet,
        field_values: &std::collections::HashMap<i32, String>,
    ) -> Result<(), AppError> {
        // Don't post if fleet is hidden
        if fleet.hidden {
            return Ok(());
        }

        let ping_format_field_repo = PingFormatFieldRepository::new(self.db);
        let category_repo = FleetCategoryRepository::new(self.db);
        let message_repo = FleetMessageRepository::new(self.db);

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

        // Build embed
        let embed = builder::build_fleet_embed(
            fleet,
            &fields,
            field_values,
            0x3498db, // Blue color for creation
            &commander_name,
            &self.app_url,
        )
        .await?;

        // Build title based on category name
        let title = format!("**.:New Upcoming {}:.**", category_data.category.name);

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

            let message = CreateMessage::new().content(&content).embed(embed.clone());

            match channel_id.send_message(&self.http, message).await {
                Ok(msg) => {
                    // Store message in database
                    message_repo
                        .create(CreateFleetMessageParam {
                            fleet_id: fleet.id,
                            channel_id: channel_id_u64,
                            message_id: msg.id.get(),
                            message_type: "creation".to_string(),
                        })
                        .await?;

                    tracing::info!(
                        "Posted fleet creation for fleet {} to channel {}",
                        fleet.id,
                        channel_id_u64
                    );
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to post fleet creation to channel {}: {}",
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

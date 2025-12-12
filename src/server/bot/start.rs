use dioxus_logger::tracing;
use sea_orm::DatabaseConnection;
use serenity::all::{Client, GatewayIntents};

use crate::server::config::Config;
use crate::server::error::AppError;

use super::handler::Handler;

/// Starts the Discord bot in a blocking manner
///
/// This function creates and starts the Discord bot client. It should be called from within
/// a tokio::spawn task since it will block until the bot shuts down.
///
/// The bot requires a DISCORD_BOT_TOKEN environment variable to be set.
///
/// # Arguments
/// - `config` - Application configuration
/// - `db` - Database connection for the bot to use
///
/// # Returns
/// - `Ok(())` if the bot starts and runs successfully
/// - `Err(AppError)` if bot initialization or connection fails
pub async fn start_bot(config: &Config, db: DatabaseConnection) -> Result<(), AppError> {
    // Configure gateway intents - what events the bot will receive
    // GUILD_MEMBERS is a privileged intent - must be enabled in Discord Developer Portal
    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MEMBERS;

    // Create the event handler with database access
    let handler = Handler::new(db);

    // Build the client
    let mut client = Client::builder(&config.discord_bot_token, intents)
        .event_handler(handler)
        .await?;

    tracing::info!("Starting Discord bot...");

    // Start the bot (this blocks until shutdown)
    client.start().await?;

    Ok(())
}

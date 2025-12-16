use dioxus_logger::tracing;
use sea_orm::DatabaseConnection;
use serenity::all::{Client, GatewayIntents};
use serenity::http::Http;
use std::sync::Arc;

use crate::server::config::Config;
use crate::server::error::AppError;

use super::handler::Handler;

/// Initializes the Discord bot client and returns the HTTP client
///
/// This function creates the bot client and extracts the HTTP client that can be shared
/// with other parts of the application. The actual bot connection is started separately.
///
/// # Arguments
/// - `config` - Application configuration
/// - `db` - Database connection for the bot to use
///
/// # Returns
/// - `Ok((Client, Arc<Http>))` - The bot client and HTTP client
/// - `Err(AppError)` if bot initialization fails
pub async fn init_bot(
    config: &Config,
    db: DatabaseConnection,
) -> Result<(Client, Arc<Http>), AppError> {
    // Configure gateway intents - what events the bot will receive
    // GUILD_MEMBERS is a privileged intent - must be enabled in Discord Developer Portal
    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_MEMBERS;

    // Create the event handler with database access
    let handler = Handler::new(db);

    // Build the client
    let client = Client::builder(&config.discord_bot_token, intents)
        .event_handler(handler)
        .await?;

    // Clone the HTTP client to share with the rest of the app
    let http = client.http.clone();

    Ok((client, http))
}

/// Starts the Discord bot (blocking)
///
/// This function starts the Discord bot client. It should be called from within
/// a tokio::spawn task since it will block until the bot shuts down.
///
/// # Arguments
/// - `client` - The Discord bot client to start
///
/// # Returns
/// - `Ok(())` if the bot runs successfully
/// - `Err(AppError)` if connection fails
pub async fn start_bot(mut client: Client) -> Result<(), AppError> {
    tracing::info!("Starting Discord bot...");

    // Start the bot (this blocks until shutdown)
    client.start().await?;

    Ok(())
}

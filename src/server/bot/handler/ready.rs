use dioxus_logger::tracing;
use serenity::all::{ActivityData, Context, Ready};

/// Handles the ready event when the bot connects to Discord
///
/// Sets the bot's activity status and logs connection information.
pub async fn handle_ready(ctx: Context, ready: Ready) {
    tracing::info!("{} is connected to Discord!", ready.user.name);

    ctx.set_activity(Some(ActivityData::custom("Tank Moonman <3")));
}

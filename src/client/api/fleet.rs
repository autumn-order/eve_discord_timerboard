use crate::{
    client::model::error::ApiError,
    model::{category::FleetCategoryDetailsDto, discord::DiscordGuildMemberDto},
};

use super::helper::{get, parse_response, send_request};

/// GET /api/guilds/{guild_id}/categories/{category_id}/details
/// Get category details including ping format fields for fleet creation
pub async fn get_category_details(
    guild_id: u64,
    category_id: i32,
) -> Result<FleetCategoryDetailsDto, ApiError> {
    let url = format!(
        "/api/guilds/{}/categories/{}/details",
        guild_id, category_id
    );
    let request = get(&url);
    let response = send_request(request).await?;
    parse_response(response).await
}

/// GET /api/guilds/{guild_id}/members
/// Get all members of a guild for FC selection
pub async fn get_guild_members(guild_id: u64) -> Result<Vec<DiscordGuildMemberDto>, ApiError> {
    let url = format!("/api/guilds/{}/members", guild_id);
    let request = get(&url);
    let response = send_request(request).await?;
    parse_response(response).await
}

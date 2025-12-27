use crate::{
    client::{
        api::helper::{
            delete, extract_error_message, get, parse_empty_response, parse_response, post,
            send_request,
        },
        model::error::ApiError,
    },
    model::{
        category::FleetCategoryListItemDto,
        discord::DiscordGuildDto,
        user::{PaginatedUsersDto, UserDto},
    },
};

pub async fn get_user() -> Result<Option<UserDto>, ApiError> {
    let url = format!("/api/auth/user");
    let response = send_request(|| get(&url)).await?;

    let status = response.status();

    match status {
        200 => {
            // Try to parse the response as UserDto
            match response.json::<UserDto>().await {
                Ok(user) => Ok(Some(user)),
                Err(e) => {
                    let message = format!("Failed to parse user data: {}", e);
                    Err(ApiError {
                        status: status.into(),
                        message,
                    })
                }
            }
        }
        404 => Ok(None),
        _ => {
            let (status, message) = extract_error_message(response).await;
            Err(ApiError { status, message })
        }
    }
}

pub async fn get_all_users(page: u64, per_page: u64) -> Result<PaginatedUsersDto, ApiError> {
    let url = format!("/api/admin/users?page={}&per_page={}", page, per_page);
    let response = send_request(|| get(&url)).await?;
    parse_response(response).await
}

pub async fn get_all_admins() -> Result<Vec<UserDto>, ApiError> {
    let response = send_request(|| get("/api/admin/admins")).await?;
    parse_response(response).await
}

pub async fn add_admin(user_id: u64) -> Result<(), ApiError> {
    let url = format!("/api/admin/admins/{}", user_id);
    let response = send_request(|| post(&url)).await?;
    parse_empty_response(response).await
}

pub async fn remove_admin(user_id: u64) -> Result<(), ApiError> {
    let url = format!("/api/admin/admins/{}", user_id);
    let response = send_request(|| delete(&url)).await?;
    parse_empty_response(response).await
}

pub async fn get_user_guilds() -> Result<Vec<DiscordGuildDto>, ApiError> {
    let response = send_request(|| get("/api/user/guilds")).await?;
    parse_response(response).await
}

pub async fn get_user_manageable_categories(
    guild_id: u64,
) -> Result<Vec<FleetCategoryListItemDto>, ApiError> {
    let url = format!("/api/user/guilds/{}/manageable-categories", guild_id);
    let response = send_request(|| get(&url)).await?;
    parse_response(response).await
}

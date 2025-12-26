use chrono::Duration;
use serde::{Deserialize, Serialize};

use crate::model::pagination::PageDto;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(utoipa::ToSchema))]
pub struct PingGroupDto {
    pub id: i32,
    pub guild_id: u64,
    pub name: String,
    pub cooldown: Option<Duration>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(utoipa::ToSchema))]
pub struct CreatePingGroupDto {
    pub name: String,
    pub cooldown: Option<Duration>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(utoipa::ToSchema))]
pub struct UpdatePingGroupDto {
    pub name: String,
    pub cooldown: Option<Duration>,
}

pub type PaginatedPingGroupsDto = PageDto<PingGroupDto>;

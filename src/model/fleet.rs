use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FleetCategoryDto {
    pub id: i32,
    pub guild_id: i64,
    pub ping_format_id: i32,
    pub name: String,
    pub ping_lead_time: Option<Duration>,
    pub ping_reminder: Option<Duration>,
    pub max_pre_ping: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFleetCategoryDto {
    pub ping_format_id: i32,
    pub name: String,
    pub ping_lead_time: Option<Duration>,
    pub ping_reminder: Option<Duration>,
    pub max_pre_ping: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFleetCategoryDto {
    pub ping_format_id: i32,
    pub name: String,
    pub ping_lead_time: Option<Duration>,
    pub ping_reminder: Option<Duration>,
    pub max_pre_ping: Option<Duration>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaginatedFleetCategoriesDto {
    pub categories: Vec<FleetCategoryDto>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

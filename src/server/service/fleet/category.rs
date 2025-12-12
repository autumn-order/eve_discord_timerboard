use chrono::Duration;
use sea_orm::DatabaseConnection;

use crate::{
    model::fleet::{FleetCategoryDto, PaginatedFleetCategoriesDto},
    server::{data::fleet::FleetCategoryRepository, error::AppError},
};

pub struct FleetCategoryService<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> FleetCategoryService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    /// Creates a new fleet category for a guild
    pub async fn create(
        &self,
        guild_id: i64,
        name: String,
        ping_lead_time: Option<Duration>,
        ping_reminder: Option<Duration>,
        max_pre_ping: Option<Duration>,
    ) -> Result<FleetCategoryDto, AppError> {
        let repo = FleetCategoryRepository::new(self.db);

        let category = repo
            .create(guild_id, name, ping_lead_time, ping_reminder, max_pre_ping)
            .await?;

        Ok(FleetCategoryDto {
            id: category.id,
            guild_id: category.guild_id,
            name: category.name,
            ping_lead_time: category.ping_cooldown.map(|s| Duration::seconds(s as i64)),
            ping_reminder: category.ping_reminder.map(|s| Duration::seconds(s as i64)),
            max_pre_ping: category.max_pre_ping.map(|s| Duration::seconds(s as i64)),
        })
    }

    /// Gets paginated fleet categories for a guild
    pub async fn get_paginated(
        &self,
        guild_id: i64,
        page: u64,
        per_page: u64,
    ) -> Result<PaginatedFleetCategoriesDto, AppError> {
        let repo = FleetCategoryRepository::new(self.db);

        let (categories, total) = repo
            .get_by_guild_id_paginated(guild_id, page, per_page)
            .await?;

        let total_pages = if per_page > 0 {
            (total as f64 / per_page as f64).ceil() as u64
        } else {
            0
        };

        Ok(PaginatedFleetCategoriesDto {
            categories: categories
                .into_iter()
                .map(|c| FleetCategoryDto {
                    id: c.id,
                    guild_id: c.guild_id,
                    name: c.name,
                    ping_lead_time: c.ping_cooldown.map(|s| Duration::seconds(s as i64)),
                    ping_reminder: c.ping_reminder.map(|s| Duration::seconds(s as i64)),
                    max_pre_ping: c.max_pre_ping.map(|s| Duration::seconds(s as i64)),
                })
                .collect(),
            total,
            page,
            per_page,
            total_pages,
        })
    }

    /// Updates a fleet category's name and duration fields
    /// Returns None if the category doesn't exist or doesn't belong to the guild
    pub async fn update(
        &self,
        id: i32,
        guild_id: i64,
        name: String,
        ping_lead_time: Option<Duration>,
        ping_reminder: Option<Duration>,
        max_pre_ping: Option<Duration>,
    ) -> Result<Option<FleetCategoryDto>, AppError> {
        let repo = FleetCategoryRepository::new(self.db);

        // Check if category exists and belongs to the guild
        if !repo.exists_in_guild(id, guild_id).await? {
            return Ok(None);
        }

        let category = repo
            .update(id, name, ping_lead_time, ping_reminder, max_pre_ping)
            .await?;

        Ok(Some(FleetCategoryDto {
            id: category.id,
            guild_id: category.guild_id,
            name: category.name,
            ping_lead_time: category.ping_cooldown.map(|s| Duration::seconds(s as i64)),
            ping_reminder: category.ping_reminder.map(|s| Duration::seconds(s as i64)),
            max_pre_ping: category.max_pre_ping.map(|s| Duration::seconds(s as i64)),
        }))
    }

    /// Deletes a fleet category
    /// Returns true if deleted, false if not found or doesn't belong to guild
    pub async fn delete(&self, id: i32, guild_id: i64) -> Result<bool, AppError> {
        let repo = FleetCategoryRepository::new(self.db);

        // Check if category exists and belongs to the guild
        if !repo.exists_in_guild(id, guild_id).await? {
            return Ok(false);
        }

        repo.delete(id).await?;

        Ok(true)
    }
}

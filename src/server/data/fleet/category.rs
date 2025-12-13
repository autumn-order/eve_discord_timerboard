use chrono::Duration;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};

pub struct FleetCategoryRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> FleetCategoryRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    /// Creates a new fleet category and returns it with related ping format
    pub async fn create(
        &self,
        guild_id: i64,
        ping_format_id: i32,
        name: String,
        ping_lead_time: Option<Duration>,
        ping_reminder: Option<Duration>,
        max_pre_ping: Option<Duration>,
    ) -> Result<
        (
            entity::fleet_category::Model,
            Option<entity::ping_format::Model>,
        ),
        DbErr,
    > {
        let category = entity::fleet_category::ActiveModel {
            guild_id: ActiveValue::Set(guild_id),
            ping_format_id: ActiveValue::Set(ping_format_id),
            name: ActiveValue::Set(name),
            ping_cooldown: ActiveValue::Set(ping_lead_time.map(|d| d.num_seconds() as i32)),
            ping_reminder: ActiveValue::Set(ping_reminder.map(|d| d.num_seconds() as i32)),
            max_pre_ping: ActiveValue::Set(max_pre_ping.map(|d| d.num_seconds() as i32)),
            ..Default::default()
        }
        .insert(self.db)
        .await?;

        // Fetch with related ping format
        let result = entity::prelude::FleetCategory::find_by_id(category.id)
            .find_also_related(entity::prelude::PingFormat)
            .one(self.db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Fleet category with id {} not found after creation",
                category.id
            )))?;

        Ok(result)
    }

    /// Gets a fleet category by ID with related ping format
    pub async fn get_by_id(
        &self,
        id: i32,
    ) -> Result<
        Option<(
            entity::fleet_category::Model,
            Option<entity::ping_format::Model>,
        )>,
        DbErr,
    > {
        entity::prelude::FleetCategory::find_by_id(id)
            .find_also_related(entity::prelude::PingFormat)
            .one(self.db)
            .await
    }

    /// Gets paginated fleet categories for a guild with related ping format
    pub async fn get_by_guild_id_paginated(
        &self,
        guild_id: i64,
        page: u64,
        per_page: u64,
    ) -> Result<
        (
            Vec<(
                entity::fleet_category::Model,
                Option<entity::ping_format::Model>,
            )>,
            u64,
        ),
        DbErr,
    > {
        let paginator = entity::prelude::FleetCategory::find()
            .find_also_related(entity::prelude::PingFormat)
            .filter(entity::fleet_category::Column::GuildId.eq(guild_id))
            .order_by_asc(entity::fleet_category::Column::Name)
            .paginate(self.db, per_page);

        let total = paginator.num_items().await?;
        let categories = paginator.fetch_page(page).await?;

        Ok((categories, total))
    }

    /// Updates a fleet category's name and duration fields and returns it with related ping format
    pub async fn update(
        &self,
        id: i32,
        ping_format_id: i32,
        name: String,
        ping_lead_time: Option<Duration>,
        ping_reminder: Option<Duration>,
        max_pre_ping: Option<Duration>,
    ) -> Result<
        (
            entity::fleet_category::Model,
            Option<entity::ping_format::Model>,
        ),
        DbErr,
    > {
        let category = entity::prelude::FleetCategory::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Fleet category with id {} not found",
                id
            )))?;

        let mut active_model: entity::fleet_category::ActiveModel = category.into();
        active_model.ping_format_id = ActiveValue::Set(ping_format_id);
        active_model.name = ActiveValue::Set(name);
        active_model.ping_cooldown =
            ActiveValue::Set(ping_lead_time.map(|d| d.num_seconds() as i32));
        active_model.ping_reminder =
            ActiveValue::Set(ping_reminder.map(|d| d.num_seconds() as i32));
        active_model.max_pre_ping = ActiveValue::Set(max_pre_ping.map(|d| d.num_seconds() as i32));

        active_model.update(self.db).await?;

        // Fetch with related ping format
        let result = entity::prelude::FleetCategory::find_by_id(id)
            .find_also_related(entity::prelude::PingFormat)
            .one(self.db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Fleet category with id {} not found after update",
                id
            )))?;

        Ok(result)
    }

    /// Deletes a fleet category
    pub async fn delete(&self, id: i32) -> Result<(), DbErr> {
        entity::prelude::FleetCategory::delete_by_id(id)
            .exec(self.db)
            .await?;

        Ok(())
    }

    /// Checks if a fleet category exists and belongs to the specified guild
    pub async fn exists_in_guild(&self, id: i32, guild_id: i64) -> Result<bool, DbErr> {
        let count = entity::prelude::FleetCategory::find()
            .filter(entity::fleet_category::Column::Id.eq(id))
            .filter(entity::fleet_category::Column::GuildId.eq(guild_id))
            .count(self.db)
            .await?;

        Ok(count > 0)
    }

    /// Gets fleet categories by ping format ID
    pub async fn get_by_ping_format_id(
        &self,
        ping_format_id: i32,
    ) -> Result<Vec<entity::fleet_category::Model>, DbErr> {
        entity::prelude::FleetCategory::find()
            .filter(entity::fleet_category::Column::PingFormatId.eq(ping_format_id))
            .order_by_asc(entity::fleet_category::Column::Name)
            .all(self.db)
            .await
    }
}

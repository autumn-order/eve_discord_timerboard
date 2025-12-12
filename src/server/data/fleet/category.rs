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

    /// Creates a new fleet category
    pub async fn create(
        &self,
        guild_id: i64,
        name: String,
    ) -> Result<entity::fleet_category::Model, DbErr> {
        entity::fleet_category::ActiveModel {
            guild_id: ActiveValue::Set(guild_id),
            name: ActiveValue::Set(name),
            ..Default::default()
        }
        .insert(self.db)
        .await
    }

    /// Gets a fleet category by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Option<entity::fleet_category::Model>, DbErr> {
        entity::prelude::FleetCategory::find_by_id(id)
            .one(self.db)
            .await
    }

    /// Gets paginated fleet categories for a guild
    pub async fn get_by_guild_id_paginated(
        &self,
        guild_id: i64,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<entity::fleet_category::Model>, u64), DbErr> {
        let paginator = entity::prelude::FleetCategory::find()
            .filter(entity::fleet_category::Column::GuildId.eq(guild_id))
            .order_by_asc(entity::fleet_category::Column::Name)
            .paginate(self.db, per_page);

        let total = paginator.num_items().await?;
        let categories = paginator.fetch_page(page).await?;

        Ok((categories, total))
    }

    /// Updates a fleet category's name
    pub async fn update(
        &self,
        id: i32,
        name: String,
    ) -> Result<entity::fleet_category::Model, DbErr> {
        let category = entity::prelude::FleetCategory::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!(
                "Fleet category with id {} not found",
                id
            )))?;

        let mut active_model: entity::fleet_category::ActiveModel = category.into();
        active_model.name = ActiveValue::Set(name);

        active_model.update(self.db).await
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
}

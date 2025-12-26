//! Ping group data repository for database operations
//!
//! Provides the `PingGroupRepository` for managing ping groups in the database.
//! Provides methods to create, get, update, and delete ping groups as well as handles
//! the conversion of database entity models into domain models for usage within services
//! & controllers.

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel, PaginatorTrait, QueryFilter,
};

use crate::server::{
    error::AppError,
    model::ping_group::{CreatePingGroupParam, PingGroup, UpdatePingGroupParam},
};

/// Repository providing database operations for ping group management.
///
/// This struct holds a reference to the database connection and provides methods
/// for creating, reading, updating, and deleting ping group records.
pub struct PingGroupRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> PingGroupRepository<'a> {
    /// Creates a new PingGroupRepository instance
    ///
    /// # Arguments
    /// - `db` - Reference to the database connection
    ///
    /// # Returns
    /// - `PingGroupRepository` - new repository instance
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    /// Creates a new ping group
    ///
    /// # Arguments
    /// - `guild_id` - The ID of the guild the ping group belongs to
    /// - `param` - Create parameters containing the ping group creation data
    ///
    /// # Returns
    /// - `Ok(PingGroup)` - The created domain model as a domain model
    /// - `Err(AppError::Database)` - Database error during insert operation
    pub async fn create(
        &self,
        guild_id: u64,
        param: CreatePingGroupParam,
    ) -> Result<PingGroup, AppError> {
        let entity = entity::prelude::PingGroup::insert(entity::ping_group::ActiveModel {
            guild_id: ActiveValue::Set(guild_id.to_string()),
            name: ActiveValue::Set(param.name),
            cooldown: ActiveValue::Set(param.cooldown.map(|d| d.num_seconds() as i32)),
            ..Default::default()
        })
        .exec_with_returning(self.db)
        .await?;

        Ok(PingGroup::from_entity(entity)?)
    }

    pub async fn list_by_guild(
        &self,
        guild_id: u64,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<PingGroup>, u64), AppError> {
        let paginator = entity::prelude::PingGroup::find()
            .filter(entity::ping_group::Column::GuildId.eq(guild_id.to_string()))
            .paginate(self.db, per_page);

        let total = paginator.num_items().await?;
        let entities = paginator.fetch_page(page).await?;

        let ping_groups = entities
            .into_iter()
            .map(PingGroup::from_entity)
            .collect::<Result<Vec<_>, _>>()?;

        Ok((ping_groups, total))
    }

    /// Counts the number of ping groups for a guild
    pub async fn count_by_guild(&self, guild_id: u64) -> Result<usize, AppError> {
        let count = entity::prelude::PingGroup::find()
            .filter(entity::ping_group::Column::GuildId.eq(guild_id.to_string()))
            .count(self.db)
            .await?;

        Ok(count as usize)
    }

    /// Updates the ping group based upon provided ID & update parameters
    ///
    /// # Arguments
    /// - `guild_id` - The ID of the guild the ping group belongs to
    /// - `id` - ID of the ping group to retrieve
    /// - `param` - Update parameters of the ping group fields to modify
    ///
    /// # Returns
    /// - `Ok(PingGroup)` - The updated ping group as a domain model
    /// - `Err(AppError::Database)` - Database error during update operation
    pub async fn update(
        &self,
        guild_id: u64,
        id: i32,
        param: UpdatePingGroupParam,
    ) -> Result<PingGroup, AppError> {
        let Some(entity) = self.find_entity_by_id(guild_id, id).await? else {
            return Err(AppError::NotFound(format!(
                "Failed to find ping group ID {} for guild ID {} while attempting update",
                id, guild_id
            )));
        };

        let mut active_model = entity.into_active_model();

        active_model.name = ActiveValue::Set(param.name);
        active_model.cooldown = ActiveValue::Set(param.cooldown.map(|d| d.num_seconds() as i32));

        let entity = active_model.update(self.db).await?;

        Ok(PingGroup::from_entity(entity)?)
    }

    /// Deletes ping group of the provided ID
    ///
    /// # Arguments
    /// - `guild_id` - The ID of the guild the ping group belongs to
    /// - `id` - The ID of the ping group to delete
    ///
    /// # Returns
    /// - `Ok(()))` - The ping group was successfully deleted
    /// - `Err(AppError::Database)` - Database error during delete operation
    pub async fn delete(&self, guild_id: u64, id: i32) -> Result<(), AppError> {
        entity::prelude::PingGroup::delete_many()
            .filter(entity::ping_group::Column::Id.eq(id))
            .filter(entity::ping_group::Column::GuildId.eq(guild_id.to_string()))
            .exec(self.db)
            .await?;

        Ok(())
    }

    /// Finds a ping group by ID within a specific guild
    ///
    /// # Arguments
    /// - `guild_id` - The ID of the guild the ping group belongs to
    /// - `id` - The ID of the ping group to retrieve
    ///
    /// # Returns
    /// - `Ok(Some(PingGroup))` - The ping group if found in the specified guild
    /// - `Ok(None)` - If no ping group exists with the given ID in the guild
    /// - `Err(AppError::Database)` - Database error during fetch operation
    pub async fn find_by_id(&self, guild_id: u64, id: i32) -> Result<Option<PingGroup>, AppError> {
        let entity = self.find_entity_by_id(guild_id, id).await?;

        entity.map(PingGroup::from_entity).transpose()
    }

    /// Helper method to find a ping group entity by ID
    async fn find_entity_by_id(
        &self,
        guild_id: u64,
        id: i32,
    ) -> Result<Option<entity::ping_group::Model>, DbErr> {
        entity::prelude::PingGroup::find()
            .filter(entity::ping_group::Column::GuildId.eq(guild_id.to_string()))
            .filter(entity::ping_group::Column::Id.eq(id))
            .one(self.db)
            .await
    }
}

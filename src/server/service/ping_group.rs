use sea_orm::DatabaseConnection;

use crate::{
    constant::PING_GROUP_LIMIT_PER_GUILD,
    server::{
        data::ping_group::PingGroupRepository,
        error::AppError,
        model::ping_group::{CreatePingGroupParam, PingGroup, UpdatePingGroupParam},
    },
};

pub struct PingGroupService<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> PingGroupService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        guild_id: u64,
        param: CreatePingGroupParam,
    ) -> Result<PingGroup, AppError> {
        let repo = PingGroupRepository::new(self.db);

        let count = repo.count_by_guild(guild_id).await?;
        if count >= PING_GROUP_LIMIT_PER_GUILD {
            return Err(AppError::BadRequest(format!(
                "Maximum ping group limit ({}) reached for this Discord server",
                PING_GROUP_LIMIT_PER_GUILD
            )));
        }

        repo.create(guild_id, param).await
    }

    pub async fn get_by_id(&self, guild_id: u64, id: i32) -> Result<PingGroup, AppError> {
        let repo = PingGroupRepository::new(self.db);

        let Some(ping_group) = repo.find_by_id(guild_id, id).await? else {
            return Err(AppError::NotFound(
                "The requested ping group does not exist".to_string(),
            ));
        };

        Ok(ping_group)
    }

    pub async fn update(
        &self,
        guild_id: u64,
        id: i32,
        param: UpdatePingGroupParam,
    ) -> Result<PingGroup, AppError> {
        PingGroupRepository::new(self.db)
            .update(guild_id, id, param)
            .await
    }

    pub async fn delete(&self, guild_id: u64, id: i32) -> Result<(), AppError> {
        PingGroupRepository::new(self.db).delete(guild_id, id).await
    }
}

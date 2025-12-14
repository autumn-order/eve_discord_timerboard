use sea_orm::DatabaseConnection;

use crate::{
    model::user::UserDto,
    server::{data::user::UserRepository, error::AppError},
};

pub struct UserService<'a> {
    pub db: &'a DatabaseConnection,
}

impl<'a> UserService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_user(&self, user_id: u64) -> Result<Option<UserDto>, AppError> {
        let user_repo = UserRepository::new(self.db);

        let Some(user_model) = user_repo.find_by_discord_id(user_id).await? else {
            return Ok(None);
        };

        let user = UserDto {
            discord_id: user_model.discord_id.parse::<u64>().map_err(|e| {
                AppError::InternalError(format!("Failed to parse discord_id: {}", e))
            })?,
            name: user_model.name,
            admin: user_model.admin,
        };

        Ok(Some(user))
    }
}

use migration::OnConflict;
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
};
use serenity::all::User as DiscordUser;

pub struct DiscordUserRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> DiscordUserRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn upsert(
        &self,
        user: DiscordUser,
        is_admin: bool,
    ) -> Result<entity::discord_user::Model, DbErr> {
        entity::prelude::DiscordUser::insert(entity::discord_user::ActiveModel {
            discord_id: ActiveValue::Set(user.id.get() as i32),
            name: ActiveValue::Set(user.name),
            admin: ActiveValue::Set(is_admin),
            ..Default::default()
        })
        // Update user name in case it may have changed since last login
        .on_conflict(
            OnConflict::column(entity::discord_user::Column::DiscordId)
                .update_columns([entity::discord_user::Column::Name])
                .to_owned(),
        )
        .exec_with_returning(self.db)
        .await
    }

    /// Checks if any admin users exist in the database.
    ///
    /// # Returns
    /// - `Ok(true)` if at least one admin user exists
    /// - `Ok(false)` if no admin users exist
    /// - `Err(DbErr)` if the database query fails
    pub async fn has_admin(&self) -> Result<bool, DbErr> {
        let admin_count = entity::prelude::DiscordUser::find()
            .filter(entity::discord_user::Column::Admin.eq(true))
            .count(self.db)
            .await?;

        Ok(admin_count > 0)
    }
}

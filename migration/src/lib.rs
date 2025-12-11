pub use sea_orm_migration::prelude::*;

mod m20251210_000001_create_user_table;
mod m20251211_000002_create_discord_guild_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251210_000001_create_user_table::Migration),
            Box::new(m20251211_000002_create_discord_guild_table::Migration),
        ]
    }
}

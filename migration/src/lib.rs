pub use sea_orm_migration::prelude::*;

mod m20251210_000001_create_alliance_table;
mod m20251210_000002_create_corporation_table;
mod m20251210_000003_create_character_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251210_000002_create_corporation_table::Migration),
            Box::new(m20251210_000001_create_alliance_table::Migration),
            Box::new(m20251210_000003_create_character_table::Migration),
        ]
    }
}

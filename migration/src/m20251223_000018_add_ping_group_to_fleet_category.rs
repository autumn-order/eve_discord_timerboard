use sea_orm_migration::{prelude::*, schema::*};

use super::m20251212_000009_create_fleet_category_table::FleetCategory;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(FleetCategory::Table)
                    .add_column(integer_null(FleetCategory::PingGroupId))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(FleetCategory::Table)
                    .drop_column(FleetCategory::PingGroupId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

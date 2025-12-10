use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Corporation::Table)
                    .if_not_exists()
                    .col(pk_auto(Corporation::Id))
                    .col(integer(Corporation::CorporationId))
                    .col(string(Corporation::CorporationName))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Corporation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Corporation {
    Table,
    Id,
    CorporationId,
    CorporationName,
}

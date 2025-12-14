use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]

pub struct Migration;

#[async_trait::async_trait]

impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(string_uniq(User::DiscordId).primary_key())
                    .col(string(User::Name))
                    .col(boolean(User::Admin))
                    .col(
                        timestamp(User::LastGuildSyncAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(User::LastRoleSyncAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]

pub enum User {
    Table,
    DiscordId,
    Name,
    Admin,
    LastGuildSyncAt,
    LastRoleSyncAt,
}

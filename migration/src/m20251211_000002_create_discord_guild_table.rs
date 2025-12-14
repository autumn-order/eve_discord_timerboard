use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]

pub struct Migration;

#[async_trait::async_trait]

impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DiscordGuild::Table)
                    .if_not_exists()
                    .col(string_uniq(DiscordGuild::GuildId).primary_key())
                    .col(string(DiscordGuild::Name))
                    .col(string_null(DiscordGuild::IconHash))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DiscordGuild::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]

pub enum DiscordGuild {
    Table,
    GuildId,
    Name,
    IconHash,
}

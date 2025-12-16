use sea_orm_migration::{prelude::*, schema::*};

use super::m20251211_000002_create_discord_guild_table::DiscordGuild;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DiscordGuildMember::Table)
                    .if_not_exists()
                    .col(string(DiscordGuildMember::UserId))
                    .col(string(DiscordGuildMember::GuildId))
                    .col(string(DiscordGuildMember::Username))
                    .col(string_null(DiscordGuildMember::Nickname))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_discord_guild_member_guild_id")
                            .from(DiscordGuildMember::Table, DiscordGuildMember::GuildId)
                            .to(DiscordGuild::Table, DiscordGuild::GuildId)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(DiscordGuildMember::UserId)
                            .col(DiscordGuildMember::GuildId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DiscordGuildMember::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DiscordGuildMember {
    Table,
    UserId,
    GuildId,
    Username,
    Nickname,
}

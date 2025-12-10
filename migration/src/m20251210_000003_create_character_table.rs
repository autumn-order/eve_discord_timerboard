use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251210_000001_create_alliance_table::Alliance;
use crate::m20251210_000002_create_corporation_table::Corporation;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Character::Table)
                    .if_not_exists()
                    .col(pk_auto(Character::Id))
                    .col(integer(Character::CharacterId))
                    .col(string(Character::CharacterName))
                    .col(integer(Character::CorporationId))
                    .col(integer_null(Character::AllianceId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_character_corporation_id")
                            .from(Character::Table, Character::CorporationId)
                            .to(Corporation::Table, Corporation::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_character_alliance_id")
                            .from(Character::Table, Character::AllianceId)
                            .to(Alliance::Table, Alliance::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Character::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Character {
    Table,
    Id,
    CharacterId,
    CharacterName,
    CorporationId,
    AllianceId,
}

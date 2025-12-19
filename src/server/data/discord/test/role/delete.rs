use super::*;

/// Tests deleting an existing Discord role.
///
/// Verifies that the repository successfully deletes a role record
/// from the database by role_id.
///
/// Expected: Ok with role deleted
#[tokio::test]
async fn deletes_role() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let _role =
        factory::discord_guild_role::create_guild_role(db, &guild.guild_id, "123456789").await?;

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.delete(123456789).await;

    assert!(result.is_ok());

    // Verify role no longer exists
    let db_role = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::RoleId.eq("123456789"))
        .one(db)
        .await?;
    assert!(db_role.is_none());

    Ok(())
}

/// Tests deleting nonexistent role.
///
/// Verifies that the repository returns Ok when attempting to delete
/// a role that doesn't exist (idempotent operation).
///
/// Expected: Ok (no error)
#[tokio::test]
async fn succeeds_for_nonexistent_role() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.delete(999999999).await;

    assert!(result.is_ok());

    Ok(())
}

/// Tests deleting role doesn't affect other roles.
///
/// Verifies that deleting one role leaves other roles in the same
/// guild intact.
///
/// Expected: Ok with only specified role deleted
#[tokio::test]
async fn deletes_only_specified_role() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let _role1 =
        factory::discord_guild_role::create_guild_role(db, &guild.guild_id, "111111111").await?;
    let _role2 =
        factory::discord_guild_role::create_guild_role(db, &guild.guild_id, "222222222").await?;
    let _role3 =
        factory::discord_guild_role::create_guild_role(db, &guild.guild_id, "333333333").await?;

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.delete(222222222).await;

    assert!(result.is_ok());

    // Verify role2 is deleted
    let db_role2 = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::RoleId.eq("222222222"))
        .one(db)
        .await?;
    assert!(db_role2.is_none());

    // Verify other roles still exist
    let db_role1 = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::RoleId.eq("111111111"))
        .one(db)
        .await?;
    assert!(db_role1.is_some());

    let db_role3 = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::RoleId.eq("333333333"))
        .one(db)
        .await?;
    assert!(db_role3.is_some());

    Ok(())
}

/// Tests deleting all roles for a guild.
///
/// Verifies that multiple roles can be deleted and the guild
/// can exist without any roles.
///
/// Expected: Ok with all roles deleted
#[tokio::test]
async fn deletes_all_roles_for_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let _role1 =
        factory::discord_guild_role::create_guild_role(db, &guild.guild_id, "111111111").await?;
    let _role2 =
        factory::discord_guild_role::create_guild_role(db, &guild.guild_id, "222222222").await?;

    let repo = DiscordGuildRoleRepository::new(db);
    repo.delete(111111111).await?;
    repo.delete(222222222).await?;

    // Verify no roles remain
    let count = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::GuildId.eq(&guild.guild_id))
        .count(db)
        .await?;
    assert_eq!(count, 0);

    // Verify guild still exists
    let db_guild = entity::prelude::DiscordGuild::find()
        .filter(entity::discord_guild::Column::GuildId.eq(&guild.guild_id))
        .one(db)
        .await?;
    assert!(db_guild.is_some());

    Ok(())
}

/// Tests deleting role multiple times is idempotent.
///
/// Verifies that calling delete on the same role_id multiple times
/// doesn't cause errors.
///
/// Expected: Ok on all delete calls
#[tokio::test]
async fn idempotent_delete() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let _role =
        factory::discord_guild_role::create_guild_role(db, &guild.guild_id, "123456789").await?;

    let repo = DiscordGuildRoleRepository::new(db);

    // Delete first time
    let result1 = repo.delete(123456789).await;
    assert!(result1.is_ok());

    // Delete second time (already deleted)
    let result2 = repo.delete(123456789).await;
    assert!(result2.is_ok());

    // Delete third time
    let result3 = repo.delete(123456789).await;
    assert!(result3.is_ok());

    Ok(())
}

/// Tests deleting role from different guilds.
///
/// Verifies that deleting a role only affects that specific role,
/// even if the same role_id exists in different guilds.
///
/// Expected: Ok with only specified role deleted
#[tokio::test]
async fn deletes_role_from_specific_guild_only() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild1 = factory::discord_guild::create_guild(db).await?;
    let guild2 = factory::discord_guild::create_guild(db).await?;

    let _role_guild1 =
        factory::discord_guild_role::create_guild_role(db, &guild1.guild_id, "123456789").await?;
    let _role_guild2 =
        factory::discord_guild_role::create_guild_role(db, &guild2.guild_id, "987654321").await?;

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.delete(123456789).await;

    assert!(result.is_ok());

    // Verify guild1's role is deleted
    let db_role1 = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::RoleId.eq("123456789"))
        .one(db)
        .await?;
    assert!(db_role1.is_none());

    // Verify guild2's role still exists
    let db_role2 = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::RoleId.eq("987654321"))
        .one(db)
        .await?;
    assert!(db_role2.is_some());

    Ok(())
}

/// Tests cascade deletion for category access roles.
///
/// Verifies that deleting a Discord role automatically deletes
/// associated category access roles due to CASCADE foreign key constraint.
///
/// Expected: Ok with role and access roles deleted
#[tokio::test]
async fn cascades_to_category_access_roles() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .with_table(entity::prelude::PingFormat)
        .with_table(entity::prelude::FleetCategory)
        .with_table(entity::prelude::FleetCategoryAccessRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let role =
        factory::discord_guild_role::create_guild_role(db, &guild.guild_id, "123456789").await?;
    // Create a ping format first since category requires it
    let ping_format = factory::ping_format::create_ping_format(db, &guild.guild_id).await?;
    let category =
        factory::fleet_category::create_category(db, &guild.guild_id, ping_format.id).await?;

    // Create category access role
    let _access_role = entity::fleet_category_access_role::ActiveModel {
        fleet_category_id: sea_orm::ActiveValue::Set(category.id),
        role_id: sea_orm::ActiveValue::Set(role.role_id.clone()),
        can_view: sea_orm::ActiveValue::Set(true),
        can_create: sea_orm::ActiveValue::Set(true),
        can_manage: sea_orm::ActiveValue::Set(true),
    }
    .insert(db)
    .await?;

    // Delete the role
    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.delete(123456789).await;

    assert!(result.is_ok());

    // Verify role is deleted
    let db_role = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::RoleId.eq("123456789"))
        .one(db)
        .await?;
    assert!(db_role.is_none());

    // Verify access role is also deleted (cascade)
    let db_access = entity::prelude::FleetCategoryAccessRole::find()
        .filter(entity::fleet_category_access_role::Column::RoleId.eq("123456789"))
        .one(db)
        .await?;
    assert!(db_access.is_none());

    // Verify category still exists
    let db_category = entity::prelude::FleetCategory::find_by_id(category.id)
        .one(db)
        .await?;
    assert!(db_category.is_some());

    Ok(())
}

/// Tests cascade deletion for ping roles.
///
/// Verifies that deleting a Discord role automatically deletes
/// associated ping roles due to CASCADE foreign key constraint.
///
/// Expected: Ok with role and ping roles deleted
#[tokio::test]
async fn cascades_to_ping_roles() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .with_table(entity::prelude::PingFormat)
        .with_table(entity::prelude::FleetCategory)
        .with_table(entity::prelude::FleetCategoryPingRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let role =
        factory::discord_guild_role::create_guild_role(db, &guild.guild_id, "123456789").await?;
    // Create a ping format first since category requires it
    let ping_format = factory::ping_format::create_ping_format(db, &guild.guild_id).await?;
    let category =
        factory::fleet_category::create_category(db, &guild.guild_id, ping_format.id).await?;

    // Create category ping role
    let _ping_role = entity::fleet_category_ping_role::ActiveModel {
        fleet_category_id: sea_orm::ActiveValue::Set(category.id),
        role_id: sea_orm::ActiveValue::Set(role.role_id.clone()),
    }
    .insert(db)
    .await?;

    // Delete the role
    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.delete(123456789).await;

    assert!(result.is_ok());

    // Verify role is deleted
    let db_role = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::RoleId.eq("123456789"))
        .one(db)
        .await?;
    assert!(db_role.is_none());

    // Verify ping role is also deleted (cascade)
    let db_ping = entity::prelude::FleetCategoryPingRole::find()
        .filter(entity::fleet_category_ping_role::Column::RoleId.eq("123456789"))
        .one(db)
        .await?;
    assert!(db_ping.is_none());

    // Verify category still exists
    let db_category = entity::prelude::FleetCategory::find_by_id(category.id)
        .one(db)
        .await?;
    assert!(db_category.is_some());

    Ok(())
}

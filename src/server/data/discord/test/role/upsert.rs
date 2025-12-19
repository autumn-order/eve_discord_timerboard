use super::*;

/// Tests upserting a new Discord role.
///
/// Verifies that the repository successfully creates a new role record
/// with the specified guild_id, role_id, name, color, and position from
/// a Serenity Role object.
///
/// Expected: Ok with role created
#[tokio::test]
async fn upserts_new_role() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let role = create_test_role(123456789, "Test Role", 0xFF5733, 5);

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.upsert(guild_id, &role).await;

    assert!(result.is_ok());
    let upserted = result.unwrap();
    assert_eq!(upserted.role_id, 123456789);
    assert_eq!(upserted.guild_id, guild_id);
    assert_eq!(upserted.name, "Test Role");
    assert_eq!(upserted.color, "#FF5733");
    assert_eq!(upserted.position, 5);

    // Verify role exists in database
    let db_role = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::RoleId.eq("123456789"))
        .one(db)
        .await?;
    assert!(db_role.is_some());

    Ok(())
}

/// Tests upserting updates existing role.
///
/// Verifies that when a role with the same role_id already exists,
/// the upsert operation updates the name, color, and position fields
/// rather than creating a duplicate.
///
/// Expected: Ok with role updated
#[tokio::test]
async fn updates_existing_role() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    // Create initial role
    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild.guild_id, "123456789")
        .name("Old Name")
        .color("#000000")
        .position(1)
        .build()
        .await?;

    // Upsert with new values
    let role = create_test_role(123456789, "New Name", 0xFF5733, 10);

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.upsert(guild_id, &role).await;

    assert!(result.is_ok());
    let upserted = result.unwrap();
    assert_eq!(upserted.name, "New Name");
    assert_eq!(upserted.color, "#FF5733");
    assert_eq!(upserted.position, 10);

    // Verify only one role exists
    let count = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::RoleId.eq("123456789"))
        .count(db)
        .await?;
    assert_eq!(count, 1);

    Ok(())
}

/// Tests upserting role with zero color.
///
/// Verifies that roles with color value 0 (default/no color in Discord)
/// are properly stored as #000000.
///
/// Expected: Ok with color #000000
#[tokio::test]
async fn upserts_role_with_zero_color() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let role = create_test_role(123456789, "No Color Role", 0, 0);

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.upsert(guild_id, &role).await;

    assert!(result.is_ok());
    let upserted = result.unwrap();
    assert_eq!(upserted.color, "#000000");

    Ok(())
}

/// Tests upserting role with maximum color value.
///
/// Verifies that roles with maximum RGB color value (0xFFFFFF - white)
/// are properly stored as #FFFFFF.
///
/// Expected: Ok with color #FFFFFF
#[tokio::test]
async fn upserts_role_with_max_color() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let role = create_test_role(123456789, "White Role", 0xFFFFFF, 0);

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.upsert(guild_id, &role).await;

    assert!(result.is_ok());
    let upserted = result.unwrap();
    assert_eq!(upserted.color, "#FFFFFF");

    Ok(())
}

/// Tests upserting role with negative position.
///
/// Verifies that roles can have negative position values (i16 range).
///
/// Expected: Ok with negative position
#[tokio::test]
async fn upserts_role_with_negative_position() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let role = create_test_role(123456789, "Negative Position Role", 0xFF5733, -5);

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.upsert(guild_id, &role).await;

    assert!(result.is_ok());
    let upserted = result.unwrap();
    assert_eq!(upserted.position, -5);

    Ok(())
}

/// Tests upserting multiple different roles for same guild.
///
/// Verifies that multiple roles with different role_ids can be upserted
/// for the same guild without conflicts.
///
/// Expected: Ok with multiple roles created
#[tokio::test]
async fn upserts_multiple_roles_for_same_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let role1 = create_test_role(111111111, "Role 1", 0xFF0000, 1);
    let role2 = create_test_role(222222222, "Role 2", 0x00FF00, 2);
    let role3 = create_test_role(333333333, "Role 3", 0x0000FF, 3);

    let repo = DiscordGuildRoleRepository::new(db);
    repo.upsert(guild_id, &role1).await?;
    repo.upsert(guild_id, &role2).await?;
    repo.upsert(guild_id, &role3).await?;

    // Verify all roles exist
    let count = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::GuildId.eq(guild_id.to_string()))
        .count(db)
        .await?;
    assert_eq!(count, 3);

    Ok(())
}

/// Tests upserting different role_ids for different guilds.
///
/// Verifies that different role_ids can be upserted to different guilds.
/// Note: role_id is globally unique in Discord, so the same role_id
/// cannot exist in multiple guilds - the second upsert would update
/// the guild_id of the first role.
///
/// Expected: Ok with separate role records for each guild
#[tokio::test]
async fn upserts_different_role_ids_for_different_guilds() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild1 = factory::discord_guild::create_guild(db).await?;
    let guild2 = factory::discord_guild::create_guild(db).await?;
    let guild1_id = guild1.guild_id.parse::<u64>().unwrap();
    let guild2_id = guild2.guild_id.parse::<u64>().unwrap();

    let role1 = create_test_role(123456789, "Guild 1 Role", 0xFF5733, 5);
    let role2 = create_test_role(987654321, "Guild 2 Role", 0x00FF00, 3);

    let repo = DiscordGuildRoleRepository::new(db);
    let result1 = repo.upsert(guild1_id, &role1).await;
    let result2 = repo.upsert(guild2_id, &role2).await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());

    // Verify both roles exist
    let count = entity::prelude::DiscordGuildRole::find().count(db).await?;
    assert_eq!(count, 2);

    // Verify each guild has its own role
    let guild1_role = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::RoleId.eq("123456789"))
        .one(db)
        .await?
        .unwrap();
    assert_eq!(guild1_role.guild_id, guild1.guild_id);

    let guild2_role = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::RoleId.eq("987654321"))
        .one(db)
        .await?
        .unwrap();
    assert_eq!(guild2_role.guild_id, guild2.guild_id);

    Ok(())
}

/// Tests upserting role with special characters in name.
///
/// Verifies that role names with special characters, emojis, and
/// Unicode are properly stored.
///
/// Expected: Ok with special characters preserved
#[tokio::test]
async fn upserts_role_with_special_characters() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let role = create_test_role(
        123456789,
        "Role 🎮 with émojis & spëcial ⭐ chars!",
        0xFF5733,
        5,
    );

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.upsert(guild_id, &role).await;

    assert!(result.is_ok());
    let upserted = result.unwrap();
    assert_eq!(upserted.name, "Role 🎮 with émojis & spëcial ⭐ chars!");

    Ok(())
}

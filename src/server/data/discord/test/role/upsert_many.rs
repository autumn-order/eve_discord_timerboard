use super::*;

/// Tests upserting multiple roles in batch.
///
/// Verifies that the repository successfully creates multiple role records
/// from a HashMap of Serenity Role objects.
///
/// Expected: Ok with all roles created
#[tokio::test]
async fn upserts_multiple_roles() -> Result<(), DbErr> {
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

    let mut roles = HashMap::new();
    roles.insert(RoleId::new(111111111), role1);
    roles.insert(RoleId::new(222222222), role2);
    roles.insert(RoleId::new(333333333), role3);

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.upsert_many(guild_id, &roles).await;

    assert!(result.is_ok());
    let upserted = result.unwrap();
    assert_eq!(upserted.len(), 3);

    // Verify all roles exist in database
    let count = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::GuildId.eq(guild_id.to_string()))
        .count(db)
        .await?;
    assert_eq!(count, 3);

    Ok(())
}

/// Tests upserting empty role HashMap.
///
/// Verifies that upserting an empty HashMap succeeds and returns
/// an empty vector without errors.
///
/// Expected: Ok with empty vector
#[tokio::test]
async fn upserts_empty_roles() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let roles = HashMap::new();

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.upsert_many(guild_id, &roles).await;

    assert!(result.is_ok());
    let upserted = result.unwrap();
    assert_eq!(upserted.len(), 0);

    Ok(())
}

/// Tests upsert_many updates existing roles.
///
/// Verifies that when some roles already exist in the database,
/// upsert_many updates them rather than creating duplicates.
///
/// Expected: Ok with roles updated
#[tokio::test]
async fn updates_existing_roles() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    // Create initial roles with old values
    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild.guild_id, "111111111")
        .name("Old Name 1")
        .color("#000000")
        .position(1)
        .build()
        .await?;

    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild.guild_id, "222222222")
        .name("Old Name 2")
        .color("#000000")
        .position(2)
        .build()
        .await?;

    // Upsert with new values
    let role1 = create_test_role(111111111, "New Name 1", 0xFF0000, 10);
    let role2 = create_test_role(222222222, "New Name 2", 0x00FF00, 20);

    let mut roles = HashMap::new();
    roles.insert(RoleId::new(111111111), role1);
    roles.insert(RoleId::new(222222222), role2);

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.upsert_many(guild_id, &roles).await;

    assert!(result.is_ok());
    let upserted = result.unwrap();
    assert_eq!(upserted.len(), 2);

    // Verify only 2 roles exist (no duplicates)
    let count = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::GuildId.eq(guild_id.to_string()))
        .count(db)
        .await?;
    assert_eq!(count, 2);

    // Verify roles were updated
    let db_role1 = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::RoleId.eq("111111111"))
        .one(db)
        .await?
        .unwrap();
    assert_eq!(db_role1.name, "New Name 1");
    assert_eq!(db_role1.color, "#FF0000");
    assert_eq!(db_role1.position, 10);

    Ok(())
}

/// Tests upsert_many with mix of new and existing roles.
///
/// Verifies that when some roles exist and others don't,
/// upsert_many correctly updates existing ones and creates new ones.
///
/// Expected: Ok with mixed create/update operations
#[tokio::test]
async fn upserts_mix_of_new_and_existing_roles() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    // Create one existing role
    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild.guild_id, "111111111")
        .name("Existing Role")
        .color("#000000")
        .position(1)
        .build()
        .await?;

    // Upsert with one existing and two new roles
    let role1 = create_test_role(111111111, "Updated Role", 0xFF0000, 10);
    let role2 = create_test_role(222222222, "New Role 1", 0x00FF00, 20);
    let role3 = create_test_role(333333333, "New Role 2", 0x0000FF, 30);

    let mut roles = HashMap::new();
    roles.insert(RoleId::new(111111111), role1);
    roles.insert(RoleId::new(222222222), role2);
    roles.insert(RoleId::new(333333333), role3);

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.upsert_many(guild_id, &roles).await;

    assert!(result.is_ok());
    let upserted = result.unwrap();
    assert_eq!(upserted.len(), 3);

    // Verify all 3 roles exist
    let count = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::GuildId.eq(guild_id.to_string()))
        .count(db)
        .await?;
    assert_eq!(count, 3);

    // Verify the existing role was updated
    let db_role1 = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::RoleId.eq("111111111"))
        .one(db)
        .await?
        .unwrap();
    assert_eq!(db_role1.name, "Updated Role");

    Ok(())
}

/// Tests upsert_many with single role.
///
/// Verifies that upsert_many works correctly with a HashMap containing
/// only one role.
///
/// Expected: Ok with single role created
#[tokio::test]
async fn upserts_single_role() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let role = create_test_role(111111111, "Single Role", 0xFF5733, 5);

    let mut roles = HashMap::new();
    roles.insert(RoleId::new(111111111), role);

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.upsert_many(guild_id, &roles).await;

    assert!(result.is_ok());
    let upserted = result.unwrap();
    assert_eq!(upserted.len(), 1);
    assert_eq!(upserted[0].role_id, 111111111);
    assert_eq!(upserted[0].name, "Single Role");

    Ok(())
}

/// Tests upsert_many preserves guild_id for all roles.
///
/// Verifies that all upserted roles have the correct guild_id
/// matching the provided guild parameter.
///
/// Expected: Ok with all roles having correct guild_id
#[tokio::test]
async fn preserves_guild_id_for_all_roles() -> Result<(), DbErr> {
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

    let mut roles = HashMap::new();
    roles.insert(RoleId::new(111111111), role1);
    roles.insert(RoleId::new(222222222), role2);

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.upsert_many(guild_id, &roles).await;

    assert!(result.is_ok());
    let upserted = result.unwrap();

    // Verify all roles have the correct guild_id
    for role in upserted {
        assert_eq!(role.guild_id, guild_id);
    }

    Ok(())
}

/// Tests upsert_many with roles having various colors.
///
/// Verifies that roles with different color values are all properly
/// stored with correct hex color codes.
///
/// Expected: Ok with all colors correctly formatted
#[tokio::test]
async fn upserts_roles_with_various_colors() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let role1 = create_test_role(111111111, "Red", 0xFF0000, 1);
    let role2 = create_test_role(222222222, "Green", 0x00FF00, 2);
    let role3 = create_test_role(333333333, "Blue", 0x0000FF, 3);
    let role4 = create_test_role(444444444, "No Color", 0, 4);

    let mut roles = HashMap::new();
    roles.insert(RoleId::new(111111111), role1);
    roles.insert(RoleId::new(222222222), role2);
    roles.insert(RoleId::new(333333333), role3);
    roles.insert(RoleId::new(444444444), role4);

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.upsert_many(guild_id, &roles).await;

    assert!(result.is_ok());
    let upserted = result.unwrap();
    assert_eq!(upserted.len(), 4);

    // Verify colors are correct
    let db_roles = entity::prelude::DiscordGuildRole::find()
        .filter(entity::discord_guild_role::Column::GuildId.eq(guild_id.to_string()))
        .all(db)
        .await?;

    let color_map: HashMap<String, String> =
        db_roles.into_iter().map(|r| (r.role_id, r.color)).collect();

    assert_eq!(color_map.get("111111111"), Some(&"#FF0000".to_string()));
    assert_eq!(color_map.get("222222222"), Some(&"#00FF00".to_string()));
    assert_eq!(color_map.get("333333333"), Some(&"#0000FF".to_string()));
    assert_eq!(color_map.get("444444444"), Some(&"#000000".to_string()));

    Ok(())
}

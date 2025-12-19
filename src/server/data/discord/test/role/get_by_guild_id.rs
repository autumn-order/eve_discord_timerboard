use super::*;

/// Tests getting roles for a guild with multiple roles.
///
/// Verifies that the repository returns all roles belonging to
/// the specified guild.
///
/// Expected: Ok with all guild roles returned
#[tokio::test]
async fn gets_all_roles_for_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;

    // Create multiple roles
    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild.guild_id, "111111111")
        .name("Role 1")
        .position(1)
        .build()
        .await?;

    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild.guild_id, "222222222")
        .name("Role 2")
        .position(2)
        .build()
        .await?;

    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild.guild_id, "333333333")
        .name("Role 3")
        .position(3)
        .build()
        .await?;

    let guild_id = guild.guild_id.parse::<u64>().unwrap();
    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.get_by_guild_id(guild_id).await;

    assert!(result.is_ok());
    let roles = result.unwrap();
    assert_eq!(roles.len(), 3);

    // Verify all role IDs are present
    let role_ids: Vec<u64> = roles.iter().map(|r| r.role_id).collect();
    assert!(role_ids.contains(&111111111));
    assert!(role_ids.contains(&222222222));
    assert!(role_ids.contains(&333333333));

    Ok(())
}

/// Tests getting roles for guild with no roles.
///
/// Verifies that the repository returns an empty vector when the
/// guild has no associated roles.
///
/// Expected: Ok with empty vector
#[tokio::test]
async fn returns_empty_for_guild_without_roles() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.get_by_guild_id(guild_id).await;

    assert!(result.is_ok());
    let roles = result.unwrap();
    assert_eq!(roles.len(), 0);

    Ok(())
}

/// Tests getting roles for nonexistent guild.
///
/// Verifies that the repository returns an empty vector when querying
/// roles for a guild_id that doesn't exist.
///
/// Expected: Ok with empty vector
#[tokio::test]
async fn returns_empty_for_nonexistent_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.get_by_guild_id(999999999).await;

    assert!(result.is_ok());
    let roles = result.unwrap();
    assert_eq!(roles.len(), 0);

    Ok(())
}

/// Tests getting roles filters by guild_id correctly.
///
/// Verifies that only roles belonging to the specified guild
/// are returned, not roles from other guilds.
///
/// Expected: Ok with only roles from specified guild
#[tokio::test]
async fn filters_by_guild_id() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild1 = factory::discord_guild::create_guild(db).await?;
    let guild2 = factory::discord_guild::create_guild(db).await?;

    // Create roles for both guilds
    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild1.guild_id, "111111111")
        .name("Guild 1 Role 1")
        .build()
        .await?;

    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild1.guild_id, "222222222")
        .name("Guild 1 Role 2")
        .build()
        .await?;

    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild2.guild_id, "333333333")
        .name("Guild 2 Role 1")
        .build()
        .await?;

    let guild1_id = guild1.guild_id.parse::<u64>().unwrap();
    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.get_by_guild_id(guild1_id).await;

    assert!(result.is_ok());
    let roles = result.unwrap();
    assert_eq!(roles.len(), 2);

    // Verify only guild1 roles are returned
    for role in &roles {
        assert_eq!(role.guild_id, guild1_id);
    }

    let role_ids: Vec<u64> = roles.iter().map(|r| r.role_id).collect();
    assert!(role_ids.contains(&111111111));
    assert!(role_ids.contains(&222222222));
    assert!(!role_ids.contains(&333333333));

    Ok(())
}

/// Tests getting roles returns complete role data.
///
/// Verifies that the returned domain models include all role properties
/// including role_id, guild_id, name, color, and position.
///
/// Expected: Ok with complete role data
#[tokio::test]
async fn returns_complete_role_data() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;

    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild.guild_id, "123456789")
        .name("Test Role")
        .color("#FF5733")
        .position(5)
        .build()
        .await?;

    let guild_id = guild.guild_id.parse::<u64>().unwrap();
    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.get_by_guild_id(guild_id).await;

    assert!(result.is_ok());
    let roles = result.unwrap();
    assert_eq!(roles.len(), 1);

    let role = &roles[0];
    assert_eq!(role.role_id, 123456789);
    assert_eq!(role.guild_id, guild_id);
    assert_eq!(role.name, "Test Role");
    assert_eq!(role.color, "#FF5733");
    assert_eq!(role.position, 5);

    Ok(())
}

/// Tests getting roles with various positions.
///
/// Verifies that roles with different position values (positive, negative, zero)
/// are all returned correctly.
///
/// Expected: Ok with all roles returned regardless of position
#[tokio::test]
async fn gets_roles_with_various_positions() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;

    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild.guild_id, "111111111")
        .name("Negative Position")
        .position(-5)
        .build()
        .await?;

    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild.guild_id, "222222222")
        .name("Zero Position")
        .position(0)
        .build()
        .await?;

    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild.guild_id, "333333333")
        .name("Positive Position")
        .position(10)
        .build()
        .await?;

    let guild_id = guild.guild_id.parse::<u64>().unwrap();
    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.get_by_guild_id(guild_id).await;

    assert!(result.is_ok());
    let roles = result.unwrap();
    assert_eq!(roles.len(), 3);

    // Verify all positions are present
    let positions: Vec<i16> = roles.iter().map(|r| r.position).collect();
    assert!(positions.contains(&-5));
    assert!(positions.contains(&0));
    assert!(positions.contains(&10));

    Ok(())
}

/// Tests getting roles with special characters in names.
///
/// Verifies that role names with special characters, emojis, and
/// Unicode are properly retrieved.
///
/// Expected: Ok with special characters preserved
#[tokio::test]
async fn gets_roles_with_special_characters() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;

    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild.guild_id, "123456789")
        .name("Role 🎮 with émojis & spëcial ⭐ chars!")
        .build()
        .await?;

    let guild_id = guild.guild_id.parse::<u64>().unwrap();
    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.get_by_guild_id(guild_id).await;

    assert!(result.is_ok());
    let roles = result.unwrap();
    assert_eq!(roles.len(), 1);
    assert_eq!(roles[0].name, "Role 🎮 with émojis & spëcial ⭐ chars!");

    Ok(())
}

/// Tests getting single role for guild.
///
/// Verifies that the method works correctly when a guild has
/// exactly one role.
///
/// Expected: Ok with single role returned
#[tokio::test]
async fn gets_single_role() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;

    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild.guild_id, "123456789")
        .name("Only Role")
        .build()
        .await?;

    let guild_id = guild.guild_id.parse::<u64>().unwrap();
    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.get_by_guild_id(guild_id).await;

    assert!(result.is_ok());
    let roles = result.unwrap();
    assert_eq!(roles.len(), 1);
    assert_eq!(roles[0].role_id, 123456789);
    assert_eq!(roles[0].name, "Only Role");

    Ok(())
}

/// Tests getting roles with various colors.
///
/// Verifies that roles with different color values are all
/// returned with correct hex color codes.
///
/// Expected: Ok with all colors correctly formatted
#[tokio::test]
async fn gets_roles_with_various_colors() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildRole)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;

    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild.guild_id, "111111111")
        .name("Red")
        .color("#FF0000")
        .build()
        .await?;

    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild.guild_id, "222222222")
        .name("Green")
        .color("#00FF00")
        .build()
        .await?;

    factory::discord_guild_role::DiscordGuildRoleFactory::new(db, &guild.guild_id, "333333333")
        .name("No Color")
        .color("")
        .build()
        .await?;

    let guild_id = guild.guild_id.parse::<u64>().unwrap();
    let repo = DiscordGuildRoleRepository::new(db);
    let result = repo.get_by_guild_id(guild_id).await;

    assert!(result.is_ok());
    let roles = result.unwrap();
    assert_eq!(roles.len(), 3);

    // Find roles by ID and verify colors
    let red_role = roles.iter().find(|r| r.role_id == 111111111).unwrap();
    assert_eq!(red_role.color, "#FF0000");

    let green_role = roles.iter().find(|r| r.role_id == 222222222).unwrap();
    assert_eq!(green_role.color, "#00FF00");

    let no_color_role = roles.iter().find(|r| r.role_id == 333333333).unwrap();
    assert_eq!(no_color_role.color, "");

    Ok(())
}

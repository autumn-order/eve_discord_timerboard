use super::*;

/// Tests getting all members for a guild.
///
/// Verifies that the repository successfully retrieves all guild member records
/// for a specific guild.
///
/// Expected: Ok with Vec of all members
#[tokio::test]
async fn returns_all_guild_members() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::User)
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildMember)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let user1 = factory::create_user(db).await?;
    let user2 = factory::create_user(db).await?;
    let user3 = factory::create_user(db).await?;
    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse().unwrap();

    let _member1 =
        factory::create_guild_member(db, user1.discord_id.parse().unwrap(), guild_id).await?;
    let _member2 =
        factory::create_guild_member(db, user2.discord_id.parse().unwrap(), guild_id).await?;
    let _member3 =
        factory::create_guild_member(db, user3.discord_id.parse().unwrap(), guild_id).await?;

    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo.get_members_by_guild(guild_id).await;

    assert!(result.is_ok());
    let members = result.unwrap();
    assert_eq!(members.len(), 3);

    // Verify all user IDs are present
    let user_ids: Vec<u64> = members.iter().map(|m| m.user_id).collect();
    assert!(user_ids.contains(&user1.discord_id.parse::<u64>().unwrap()));
    assert!(user_ids.contains(&user2.discord_id.parse::<u64>().unwrap()));
    assert!(user_ids.contains(&user3.discord_id.parse::<u64>().unwrap()));

    Ok(())
}

/// Tests getting members returns empty list for guild with no members.
///
/// Verifies that the repository returns an empty vector when the guild
/// has no members.
///
/// Expected: Ok with empty Vec
#[tokio::test]
async fn returns_empty_for_guild_with_no_members() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::User)
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildMember)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;

    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo
        .get_members_by_guild(guild.guild_id.parse().unwrap())
        .await;

    assert!(result.is_ok());
    let members = result.unwrap();
    assert_eq!(members.len(), 0);

    Ok(())
}

/// Tests getting members for nonexistent guild.
///
/// Verifies that the repository returns an empty vector when the guild
/// doesn't exist.
///
/// Expected: Ok with empty Vec
#[tokio::test]
async fn returns_empty_for_nonexistent_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::User)
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildMember)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo.get_members_by_guild(999999999).await;

    assert!(result.is_ok());
    let members = result.unwrap();
    assert_eq!(members.len(), 0);

    Ok(())
}

/// Tests getting members only returns members from specified guild.
///
/// Verifies that the repository only returns members for the requested guild,
/// not members from other guilds.
///
/// Expected: Ok with only members from specified guild
#[tokio::test]
async fn returns_only_members_from_specified_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::User)
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildMember)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let user1 = factory::create_user(db).await?;
    let user2 = factory::create_user(db).await?;
    let user3 = factory::create_user(db).await?;
    let guild1 = factory::create_guild(db).await?;
    let guild2 = factory::create_guild(db).await?;

    let guild1_id = guild1.guild_id.parse().unwrap();
    let guild2_id = guild2.guild_id.parse().unwrap();

    // Create members in guild1
    let _member1 =
        factory::create_guild_member(db, user1.discord_id.parse().unwrap(), guild1_id).await?;
    let _member2 =
        factory::create_guild_member(db, user2.discord_id.parse().unwrap(), guild1_id).await?;

    // Create member in guild2
    let _member3 =
        factory::create_guild_member(db, user3.discord_id.parse().unwrap(), guild2_id).await?;

    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo.get_members_by_guild(guild1_id).await;

    assert!(result.is_ok());
    let members = result.unwrap();
    assert_eq!(members.len(), 2);

    // Verify only guild1 members are returned
    let user_ids: Vec<u64> = members.iter().map(|m| m.user_id).collect();
    assert!(user_ids.contains(&user1.discord_id.parse::<u64>().unwrap()));
    assert!(user_ids.contains(&user2.discord_id.parse::<u64>().unwrap()));
    assert!(!user_ids.contains(&user3.discord_id.parse::<u64>().unwrap()));

    // All members should be from guild1
    for member in members {
        assert_eq!(member.guild_id, guild1_id);
    }

    Ok(())
}

/// Tests getting members with nicknames.
///
/// Verifies that the repository correctly retrieves member nicknames.
///
/// Expected: Ok with members containing nicknames
#[tokio::test]
async fn returns_members_with_nicknames() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::User)
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildMember)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let user1 = factory::create_user(db).await?;
    let user2 = factory::create_user(db).await?;
    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse().unwrap();

    let _member1 = factory::create_guild_member_with_nickname(
        db,
        user1.discord_id.parse().unwrap(),
        guild_id,
        "Nickname1",
    )
    .await?;
    let _member2 =
        factory::create_guild_member(db, user2.discord_id.parse().unwrap(), guild_id).await?;

    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo.get_members_by_guild(guild_id).await;

    assert!(result.is_ok());
    let members = result.unwrap();
    assert_eq!(members.len(), 2);

    // Find member with nickname
    let member_with_nick = members
        .iter()
        .find(|m| m.user_id == user1.discord_id.parse::<u64>().unwrap())
        .unwrap();
    assert_eq!(member_with_nick.nickname, Some("Nickname1".to_string()));

    // Find member without nickname
    let member_without_nick = members
        .iter()
        .find(|m| m.user_id == user2.discord_id.parse::<u64>().unwrap())
        .unwrap();
    assert!(member_without_nick.nickname.is_none());

    Ok(())
}

/// Tests getting members after some are deleted.
///
/// Verifies that the repository only returns current members after
/// some members have been deleted.
///
/// Expected: Ok with only remaining members
#[tokio::test]
async fn returns_only_remaining_members_after_deletion() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::User)
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildMember)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let user1 = factory::create_user(db).await?;
    let user2 = factory::create_user(db).await?;
    let user3 = factory::create_user(db).await?;
    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse().unwrap();

    let _member1 =
        factory::create_guild_member(db, user1.discord_id.parse().unwrap(), guild_id).await?;
    let _member2 =
        factory::create_guild_member(db, user2.discord_id.parse().unwrap(), guild_id).await?;
    let _member3 =
        factory::create_guild_member(db, user3.discord_id.parse().unwrap(), guild_id).await?;

    // Delete one member
    let repo = DiscordGuildMemberRepository::new(db);
    repo.delete(user2.discord_id.parse().unwrap(), guild_id)
        .await?;

    let result = repo.get_members_by_guild(guild_id).await;

    assert!(result.is_ok());
    let members = result.unwrap();
    assert_eq!(members.len(), 2);

    // Verify deleted member is not in results
    let user_ids: Vec<u64> = members.iter().map(|m| m.user_id).collect();
    assert!(user_ids.contains(&user1.discord_id.parse::<u64>().unwrap()));
    assert!(!user_ids.contains(&user2.discord_id.parse::<u64>().unwrap()));
    assert!(user_ids.contains(&user3.discord_id.parse::<u64>().unwrap()));

    Ok(())
}

/// Tests getting members for guild with single member.
///
/// Verifies that the repository correctly handles guilds with only one member.
///
/// Expected: Ok with single member
#[tokio::test]
async fn returns_single_member() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::User)
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildMember)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let user = factory::create_user(db).await?;
    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse().unwrap();

    let _member =
        factory::create_guild_member(db, user.discord_id.parse().unwrap(), guild_id).await?;

    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo.get_members_by_guild(guild_id).await;

    assert!(result.is_ok());
    let members = result.unwrap();
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].user_id, user.discord_id.parse::<u64>().unwrap());

    Ok(())
}

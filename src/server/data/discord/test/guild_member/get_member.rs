use super::*;

/// Tests getting an existing guild member.
///
/// Verifies that the repository successfully retrieves a guild member record
/// by user ID and guild ID.
///
/// Expected: Ok(Some(member))
#[tokio::test]
async fn returns_existing_member() -> Result<(), DbErr> {
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
    let _member = factory::create_guild_member(
        db,
        user.discord_id.parse().unwrap(),
        guild.guild_id.parse().unwrap(),
    )
    .await?;

    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo
        .get_member(
            user.discord_id.parse().unwrap(),
            guild.guild_id.parse().unwrap(),
        )
        .await;

    assert!(result.is_ok());
    let member = result.unwrap();
    assert!(member.is_some());

    let member = member.unwrap();
    assert_eq!(member.user_id, user.discord_id.parse::<u64>().unwrap());
    assert_eq!(member.guild_id, guild.guild_id.parse::<u64>().unwrap());

    Ok(())
}

/// Tests getting a member with nickname.
///
/// Verifies that the repository correctly retrieves a member's nickname.
///
/// Expected: Ok(Some(member)) with nickname
#[tokio::test]
async fn returns_member_with_nickname() -> Result<(), DbErr> {
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
    let _member = factory::create_guild_member_with_nickname(
        db,
        user.discord_id.parse().unwrap(),
        guild.guild_id.parse().unwrap(),
        "CoolNickname",
    )
    .await?;

    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo
        .get_member(
            user.discord_id.parse().unwrap(),
            guild.guild_id.parse().unwrap(),
        )
        .await;

    assert!(result.is_ok());
    let member = result.unwrap();
    assert!(member.is_some());

    let member = member.unwrap();
    assert_eq!(member.nickname, Some("CoolNickname".to_string()));

    Ok(())
}

/// Tests getting nonexistent member.
///
/// Verifies that the repository returns None when the member doesn't exist.
///
/// Expected: Ok(None)
#[tokio::test]
async fn returns_none_for_nonexistent_member() -> Result<(), DbErr> {
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

    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo
        .get_member(
            user.discord_id.parse().unwrap(),
            guild.guild_id.parse().unwrap(),
        )
        .await;

    assert!(result.is_ok());
    let member = result.unwrap();
    assert!(member.is_none());

    Ok(())
}

/// Tests getting member with nonexistent user.
///
/// Verifies that the repository returns None when the user doesn't exist.
///
/// Expected: Ok(None)
#[tokio::test]
async fn returns_none_for_nonexistent_user() -> Result<(), DbErr> {
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
        .get_member(999999999, guild.guild_id.parse().unwrap())
        .await;

    assert!(result.is_ok());
    let member = result.unwrap();
    assert!(member.is_none());

    Ok(())
}

/// Tests getting member with nonexistent guild.
///
/// Verifies that the repository returns None when the guild doesn't exist.
///
/// Expected: Ok(None)
#[tokio::test]
async fn returns_none_for_nonexistent_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::User)
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildMember)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let user = factory::create_user(db).await?;

    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo
        .get_member(user.discord_id.parse().unwrap(), 999999999)
        .await;

    assert!(result.is_ok());
    let member = result.unwrap();
    assert!(member.is_none());

    Ok(())
}

/// Tests getting correct member when user is in multiple guilds.
///
/// Verifies that the repository returns the correct member record when
/// the same user is a member of multiple guilds.
///
/// Expected: Ok(Some(member)) for the correct guild
#[tokio::test]
async fn returns_correct_member_for_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::User)
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildMember)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let user = factory::create_user(db).await?;
    let guild1 = factory::create_guild(db).await?;
    let guild2 = factory::create_guild(db).await?;
    let user_id = user.discord_id.parse().unwrap();

    // Create member in both guilds with different nicknames
    let _member1 = factory::create_guild_member_with_nickname(
        db,
        user_id,
        guild1.guild_id.parse().unwrap(),
        "NickInGuild1",
    )
    .await?;
    let _member2 = factory::create_guild_member_with_nickname(
        db,
        user_id,
        guild2.guild_id.parse().unwrap(),
        "NickInGuild2",
    )
    .await?;

    let repo = DiscordGuildMemberRepository::new(db);

    // Get member in guild1
    let result1 = repo
        .get_member(user_id, guild1.guild_id.parse().unwrap())
        .await;
    assert!(result1.is_ok());
    let member1 = result1.unwrap().unwrap();
    assert_eq!(member1.nickname, Some("NickInGuild1".to_string()));

    // Get member in guild2
    let result2 = repo
        .get_member(user_id, guild2.guild_id.parse().unwrap())
        .await;
    assert!(result2.is_ok());
    let member2 = result2.unwrap().unwrap();
    assert_eq!(member2.nickname, Some("NickInGuild2".to_string()));

    Ok(())
}

/// Tests getting member doesn't return members from other guilds.
///
/// Verifies that querying for a user in a guild where they aren't a member
/// returns None, even if they are a member of other guilds.
///
/// Expected: Ok(None) when user is not in specified guild
#[tokio::test]
async fn returns_none_for_wrong_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::User)
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildMember)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let user = factory::create_user(db).await?;
    let guild1 = factory::create_guild(db).await?;
    let guild2 = factory::create_guild(db).await?;
    let user_id = user.discord_id.parse().unwrap();

    // Create member only in guild1
    let _member =
        factory::create_guild_member(db, user_id, guild1.guild_id.parse().unwrap()).await?;

    let repo = DiscordGuildMemberRepository::new(db);

    // Try to get member in guild2 (where they aren't a member)
    let result = repo
        .get_member(user_id, guild2.guild_id.parse().unwrap())
        .await;

    assert!(result.is_ok());
    let member = result.unwrap();
    assert!(member.is_none());

    Ok(())
}

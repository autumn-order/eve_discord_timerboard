use super::*;

/// Tests creating a new guild member.
///
/// Verifies that the repository successfully creates a new guild member record
/// with the specified user ID, guild ID, username, and no nickname.
///
/// Expected: Ok with member created
#[tokio::test]
async fn creates_new_member() -> Result<(), DbErr> {
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
        .upsert(
            user.discord_id.parse().unwrap(),
            guild.guild_id.parse().unwrap(),
            "TestUser".to_string(),
            None,
        )
        .await;

    assert!(result.is_ok());
    let member = result.unwrap();
    assert_eq!(member.user_id, user.discord_id.parse::<u64>().unwrap());
    assert_eq!(member.guild_id, guild.guild_id.parse::<u64>().unwrap());
    assert_eq!(member.username, "TestUser");
    assert!(member.nickname.is_none());

    // Verify member exists in database
    let db_member = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::UserId.eq(&user.discord_id))
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild.guild_id))
        .one(db)
        .await?;
    assert!(db_member.is_some());

    Ok(())
}

/// Tests creating a new guild member with nickname.
///
/// Verifies that the repository successfully creates a new guild member record
/// with a guild-specific nickname.
///
/// Expected: Ok with member created with nickname
#[tokio::test]
async fn creates_new_member_with_nickname() -> Result<(), DbErr> {
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
        .upsert(
            user.discord_id.parse().unwrap(),
            guild.guild_id.parse().unwrap(),
            "TestUser".to_string(),
            Some("CoolNickname".to_string()),
        )
        .await;

    assert!(result.is_ok());
    let member = result.unwrap();
    assert_eq!(member.username, "TestUser");
    assert_eq!(member.nickname, Some("CoolNickname".to_string()));

    Ok(())
}

/// Tests updating an existing member's username.
///
/// Verifies that upserting an existing member updates their username
/// while preserving the guild and user relationship.
///
/// Expected: Ok with username updated
#[tokio::test]
async fn updates_existing_member_username() -> Result<(), DbErr> {
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
    let user_id = user.discord_id.parse().unwrap();
    let guild_id = guild.guild_id.parse().unwrap();

    let repo = DiscordGuildMemberRepository::new(db);

    // Create initial member
    repo.upsert(user_id, guild_id, "OriginalName".to_string(), None)
        .await?;

    // Update username
    let result = repo
        .upsert(user_id, guild_id, "UpdatedName".to_string(), None)
        .await;

    assert!(result.is_ok());
    let member = result.unwrap();
    assert_eq!(member.username, "UpdatedName");
    assert!(member.nickname.is_none());

    // Verify only one record exists
    let count = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::UserId.eq(&user.discord_id))
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild.guild_id))
        .count(db)
        .await?;
    assert_eq!(count, 1);

    Ok(())
}

/// Tests updating an existing member's nickname.
///
/// Verifies that upserting an existing member updates their nickname
/// to a new value.
///
/// Expected: Ok with nickname updated
#[tokio::test]
async fn updates_existing_member_nickname() -> Result<(), DbErr> {
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
    let user_id = user.discord_id.parse().unwrap();
    let guild_id = guild.guild_id.parse().unwrap();

    let repo = DiscordGuildMemberRepository::new(db);

    // Create initial member with nickname
    repo.upsert(
        user_id,
        guild_id,
        "TestUser".to_string(),
        Some("OldNickname".to_string()),
    )
    .await?;

    // Update nickname
    let result = repo
        .upsert(
            user_id,
            guild_id,
            "TestUser".to_string(),
            Some("NewNickname".to_string()),
        )
        .await;

    assert!(result.is_ok());
    let member = result.unwrap();
    assert_eq!(member.username, "TestUser");
    assert_eq!(member.nickname, Some("NewNickname".to_string()));

    Ok(())
}

/// Tests removing nickname on update.
///
/// Verifies that upserting with None nickname removes an existing nickname.
///
/// Expected: Ok with nickname removed
#[tokio::test]
async fn removes_nickname_on_update() -> Result<(), DbErr> {
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
    let user_id = user.discord_id.parse().unwrap();
    let guild_id = guild.guild_id.parse().unwrap();

    let repo = DiscordGuildMemberRepository::new(db);

    // Create initial member with nickname
    repo.upsert(
        user_id,
        guild_id,
        "TestUser".to_string(),
        Some("Nickname".to_string()),
    )
    .await?;

    // Remove nickname
    let result = repo
        .upsert(user_id, guild_id, "TestUser".to_string(), None)
        .await;

    assert!(result.is_ok());
    let member = result.unwrap();
    assert_eq!(member.username, "TestUser");
    assert!(member.nickname.is_none());

    Ok(())
}

/// Tests adding nickname to member without one.
///
/// Verifies that upserting with Some nickname adds a nickname to a member
/// that previously had none.
///
/// Expected: Ok with nickname added
#[tokio::test]
async fn adds_nickname_to_member_without_one() -> Result<(), DbErr> {
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
    let user_id = user.discord_id.parse().unwrap();
    let guild_id = guild.guild_id.parse().unwrap();

    let repo = DiscordGuildMemberRepository::new(db);

    // Create initial member without nickname
    repo.upsert(user_id, guild_id, "TestUser".to_string(), None)
        .await?;

    // Add nickname
    let result = repo
        .upsert(
            user_id,
            guild_id,
            "TestUser".to_string(),
            Some("NewNickname".to_string()),
        )
        .await;

    assert!(result.is_ok());
    let member = result.unwrap();
    assert_eq!(member.username, "TestUser");
    assert_eq!(member.nickname, Some("NewNickname".to_string()));

    Ok(())
}

/// Tests upserting same user in different guilds.
///
/// Verifies that the same user can be a member of multiple guilds
/// with different usernames and nicknames in each.
///
/// Expected: Ok with separate member records per guild
#[tokio::test]
async fn creates_separate_records_per_guild() -> Result<(), DbErr> {
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

    let repo = DiscordGuildMemberRepository::new(db);

    // Create member in guild1
    let result1 = repo
        .upsert(
            user_id,
            guild1.guild_id.parse().unwrap(),
            "UserInGuild1".to_string(),
            Some("Nick1".to_string()),
        )
        .await;

    // Create member in guild2
    let result2 = repo
        .upsert(
            user_id,
            guild2.guild_id.parse().unwrap(),
            "UserInGuild2".to_string(),
            Some("Nick2".to_string()),
        )
        .await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());

    let member1 = result1.unwrap();
    let member2 = result2.unwrap();

    assert_eq!(member1.username, "UserInGuild1");
    assert_eq!(member1.nickname, Some("Nick1".to_string()));
    assert_eq!(member2.username, "UserInGuild2");
    assert_eq!(member2.nickname, Some("Nick2".to_string()));

    // Verify both records exist
    let count = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::UserId.eq(&user.discord_id))
        .count(db)
        .await?;
    assert_eq!(count, 2);

    Ok(())
}

/// Tests upsert is idempotent.
///
/// Verifies that calling upsert multiple times with the same data
/// doesn't create duplicates or cause errors.
///
/// Expected: Ok with single record
#[tokio::test]
async fn idempotent_upsert() -> Result<(), DbErr> {
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
    let user_id = user.discord_id.parse().unwrap();
    let guild_id = guild.guild_id.parse().unwrap();

    let repo = DiscordGuildMemberRepository::new(db);

    // Upsert three times with same data
    let result1 = repo
        .upsert(user_id, guild_id, "TestUser".to_string(), None)
        .await;
    let result2 = repo
        .upsert(user_id, guild_id, "TestUser".to_string(), None)
        .await;
    let result3 = repo
        .upsert(user_id, guild_id, "TestUser".to_string(), None)
        .await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());

    // Verify only one record exists
    let count = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::UserId.eq(&user.discord_id))
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild.guild_id))
        .count(db)
        .await?;
    assert_eq!(count, 1);

    Ok(())
}

/// Tests upserting with nonexistent user succeeds.
///
/// Verifies that creating a member with a user that doesn't exist in the
/// User table succeeds, as guild members track ALL Discord users, not just
/// those with application accounts.
///
/// Expected: Ok - no foreign key constraint on user_id
#[tokio::test]
async fn succeeds_for_nonexistent_user() -> Result<(), DbErr> {
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
        .upsert(
            999999999,
            guild.guild_id.parse().unwrap(),
            "TestUser".to_string(),
            None,
        )
        .await;

    assert!(result.is_ok());
    let member = result.unwrap();
    assert_eq!(member.user_id, 999999999);

    Ok(())
}

/// Tests upserting with nonexistent guild fails.
///
/// Verifies that attempting to create a member with a guild that doesn't
/// exist results in a database foreign key error.
///
/// Expected: Err with foreign key constraint violation
#[tokio::test]
async fn fails_for_nonexistent_guild() -> Result<(), DbErr> {
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
        .upsert(
            user.discord_id.parse().unwrap(),
            999999999,
            "TestUser".to_string(),
            None,
        )
        .await;

    assert!(result.is_err());

    Ok(())
}

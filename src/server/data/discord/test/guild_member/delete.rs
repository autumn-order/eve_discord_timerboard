use super::*;

/// Tests deleting an existing guild member.
///
/// Verifies that the repository successfully deletes a guild member record
/// from the database.
///
/// Expected: Ok with member deleted
#[tokio::test]
async fn deletes_member() -> Result<(), DbErr> {
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
        .delete(
            user.discord_id.parse().unwrap(),
            guild.guild_id.parse().unwrap(),
        )
        .await;

    assert!(result.is_ok());

    // Verify member no longer exists
    let db_member = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::UserId.eq(&user.discord_id))
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild.guild_id))
        .one(db)
        .await?;
    assert!(db_member.is_none());

    Ok(())
}

/// Tests deleting nonexistent member.
///
/// Verifies that the repository returns Ok when attempting to delete
/// a member that doesn't exist (idempotent operation).
///
/// Expected: Ok (no error)
#[tokio::test]
async fn succeeds_for_nonexistent_member() -> Result<(), DbErr> {
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
        .delete(
            user.discord_id.parse().unwrap(),
            guild.guild_id.parse().unwrap(),
        )
        .await;

    assert!(result.is_ok());

    Ok(())
}

/// Tests deleting member doesn't affect other members.
///
/// Verifies that deleting one member doesn't affect other members
/// in the same guild or the same user in other guilds.
///
/// Expected: Ok with only specified member deleted
#[tokio::test]
async fn deletes_only_specified_member() -> Result<(), DbErr> {
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
    let guild1 = factory::create_guild(db).await?;
    let guild2 = factory::create_guild(db).await?;

    let user1_id = user1.discord_id.parse().unwrap();
    let user2_id = user2.discord_id.parse().unwrap();
    let guild1_id = guild1.guild_id.parse().unwrap();
    let guild2_id = guild2.guild_id.parse().unwrap();

    // Create multiple members
    let _member1 = factory::create_guild_member(db, user1_id, guild1_id).await?;
    let _member2 = factory::create_guild_member(db, user2_id, guild1_id).await?;
    let _member3 = factory::create_guild_member(db, user1_id, guild2_id).await?;

    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo.delete(user1_id, guild1_id).await;

    assert!(result.is_ok());

    // Verify user1 in guild1 is deleted
    let db_member1 = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::UserId.eq(&user1.discord_id))
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild1.guild_id))
        .one(db)
        .await?;
    assert!(db_member1.is_none());

    // Verify user2 in guild1 still exists
    let db_member2 = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::UserId.eq(&user2.discord_id))
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild1.guild_id))
        .one(db)
        .await?;
    assert!(db_member2.is_some());

    // Verify user1 in guild2 still exists
    let db_member3 = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::UserId.eq(&user1.discord_id))
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild2.guild_id))
        .one(db)
        .await?;
    assert!(db_member3.is_some());

    Ok(())
}

/// Tests delete is idempotent.
///
/// Verifies that calling delete multiple times on the same member
/// doesn't cause errors.
///
/// Expected: Ok on all delete calls
#[tokio::test]
async fn idempotent_delete() -> Result<(), DbErr> {
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
    let user_id = user.discord_id.parse().unwrap();
    let guild_id = guild.guild_id.parse().unwrap();

    // Delete first time
    let result1 = repo.delete(user_id, guild_id).await;
    assert!(result1.is_ok());

    // Delete second time (already deleted)
    let result2 = repo.delete(user_id, guild_id).await;
    assert!(result2.is_ok());

    // Delete third time
    let result3 = repo.delete(user_id, guild_id).await;
    assert!(result3.is_ok());

    Ok(())
}

/// Tests deleting member with nonexistent user.
///
/// Verifies that attempting to delete a member for a user that doesn't
/// exist succeeds (no-op).
///
/// Expected: Ok (no error)
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
        .delete(999999999, guild.guild_id.parse().unwrap())
        .await;

    assert!(result.is_ok());

    Ok(())
}

/// Tests deleting member with nonexistent guild.
///
/// Verifies that attempting to delete a member for a guild that doesn't
/// exist succeeds (no-op).
///
/// Expected: Ok (no error)
#[tokio::test]
async fn succeeds_for_nonexistent_guild() -> Result<(), DbErr> {
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
        .delete(user.discord_id.parse().unwrap(), 999999999)
        .await;

    assert!(result.is_ok());

    Ok(())
}

/// Tests deleting all members from a guild individually.
///
/// Verifies that multiple members can be deleted individually and
/// the guild can exist without any members.
///
/// Expected: Ok with all members deleted
#[tokio::test]
async fn deletes_all_guild_members_individually() -> Result<(), DbErr> {
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
    repo.delete(user1.discord_id.parse().unwrap(), guild_id)
        .await?;
    repo.delete(user2.discord_id.parse().unwrap(), guild_id)
        .await?;
    repo.delete(user3.discord_id.parse().unwrap(), guild_id)
        .await?;

    // Verify no members remain
    let count = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild.guild_id))
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

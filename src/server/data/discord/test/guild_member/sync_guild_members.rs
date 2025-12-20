use super::*;

/// Tests syncing guild members with new members.
///
/// Verifies that the repository successfully replaces all existing members
/// with a new set of members for the guild.
///
/// Expected: Ok with all old members deleted and new ones created
#[tokio::test]
async fn syncs_guild_members() -> Result<(), DbErr> {
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
    let user4 = factory::create_user(db).await?;
    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse().unwrap();

    // Create initial members (user1, user2)
    let _member1 =
        factory::create_guild_member(db, user1.discord_id.parse().unwrap(), guild_id).await?;
    let _member2 =
        factory::create_guild_member(db, user2.discord_id.parse().unwrap(), guild_id).await?;

    // Sync to new members (user3, user4)
    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo
        .sync_guild_members(
            guild_id,
            &[
                (user3.discord_id.parse().unwrap(), "User3".to_string(), None),
                (
                    user4.discord_id.parse().unwrap(),
                    "User4".to_string(),
                    Some("Nick4".to_string()),
                ),
            ],
        )
        .await;

    assert!(result.is_ok());

    // Verify old members are deleted
    let member1 = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::UserId.eq(&user1.discord_id))
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild.guild_id))
        .one(db)
        .await?;
    assert!(member1.is_none());

    let member2 = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::UserId.eq(&user2.discord_id))
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild.guild_id))
        .one(db)
        .await?;
    assert!(member2.is_none());

    // Verify new members exist
    let member3 = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::UserId.eq(&user3.discord_id))
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild.guild_id))
        .one(db)
        .await?;
    assert!(member3.is_some());

    let member4 = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::UserId.eq(&user4.discord_id))
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild.guild_id))
        .one(db)
        .await?;
    assert!(member4.is_some());
    assert_eq!(member4.unwrap().nickname, Some("Nick4".to_string()));

    // Verify total count
    let count = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild.guild_id))
        .count(db)
        .await?;
    assert_eq!(count, 2);

    Ok(())
}

/// Tests syncing to empty member list.
///
/// Verifies that syncing with an empty member list removes all existing
/// members from the guild.
///
/// Expected: Ok with all members deleted
#[tokio::test]
async fn syncs_to_empty_member_list() -> Result<(), DbErr> {
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

    // Create initial members
    let _member1 =
        factory::create_guild_member(db, user1.discord_id.parse().unwrap(), guild_id).await?;
    let _member2 =
        factory::create_guild_member(db, user2.discord_id.parse().unwrap(), guild_id).await?;

    // Sync to empty list
    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo.sync_guild_members(guild_id, &[]).await;

    assert!(result.is_ok());

    // Verify all members are deleted
    let count = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild.guild_id))
        .count(db)
        .await?;
    assert_eq!(count, 0);

    Ok(())
}

/// Tests syncing from empty member list.
///
/// Verifies that syncing works correctly when the guild starts with
/// no members.
///
/// Expected: Ok with new members created
#[tokio::test]
async fn syncs_from_empty_member_list() -> Result<(), DbErr> {
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

    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo
        .sync_guild_members(
            guild_id,
            &[
                (user1.discord_id.parse().unwrap(), "User1".to_string(), None),
                (user2.discord_id.parse().unwrap(), "User2".to_string(), None),
            ],
        )
        .await;

    assert!(result.is_ok());

    // Verify new members exist
    let count = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild.guild_id))
        .count(db)
        .await?;
    assert_eq!(count, 2);

    Ok(())
}

/// Tests syncing with same members (no-op).
///
/// Verifies that syncing with the same members the guild already has
/// results in the same state (delete then re-create).
///
/// Expected: Ok with same members maintained
#[tokio::test]
async fn syncs_with_same_members() -> Result<(), DbErr> {
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

    // Create initial members
    let _member1 =
        factory::create_guild_member(db, user1.discord_id.parse().unwrap(), guild_id).await?;
    let _member2 =
        factory::create_guild_member(db, user2.discord_id.parse().unwrap(), guild_id).await?;

    // Sync with same members
    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo
        .sync_guild_members(
            guild_id,
            &[
                (user1.discord_id.parse().unwrap(), "User1".to_string(), None),
                (user2.discord_id.parse().unwrap(), "User2".to_string(), None),
            ],
        )
        .await;

    assert!(result.is_ok());

    // Verify members still exist
    let count = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild.guild_id))
        .count(db)
        .await?;
    assert_eq!(count, 2);

    Ok(())
}

/// Tests syncing doesn't affect other guilds.
///
/// Verifies that syncing one guild's members doesn't affect members
/// of other guilds.
///
/// Expected: Ok with only specified guild's members changed
#[tokio::test]
async fn syncs_only_specified_guild() -> Result<(), DbErr> {
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

    // Create initial members in both guilds
    let _member1 =
        factory::create_guild_member(db, user1.discord_id.parse().unwrap(), guild1_id).await?;
    let _member2 =
        factory::create_guild_member(db, user2.discord_id.parse().unwrap(), guild2_id).await?;

    // Sync guild1 to user3
    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo
        .sync_guild_members(
            guild1_id,
            &[(user3.discord_id.parse().unwrap(), "User3".to_string(), None)],
        )
        .await;

    assert!(result.is_ok());

    // Verify guild1 members are synced
    let count1 = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild1.guild_id))
        .count(db)
        .await?;
    assert_eq!(count1, 1);

    let member1 = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild1.guild_id))
        .one(db)
        .await?
        .unwrap();
    assert_eq!(member1.user_id, user3.discord_id);

    // Verify guild2 members are unchanged
    let count2 = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild2.guild_id))
        .count(db)
        .await?;
    assert_eq!(count2, 1);

    let member2 = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild2.guild_id))
        .one(db)
        .await?
        .unwrap();
    assert_eq!(member2.user_id, user2.discord_id);

    Ok(())
}

/// Tests syncing with partial overlap.
///
/// Verifies that syncing correctly handles cases where some members
/// are kept and some are added/removed.
///
/// Expected: Ok with correct final state
#[tokio::test]
async fn syncs_with_partial_overlap() -> Result<(), DbErr> {
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
    let user4 = factory::create_user(db).await?;
    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse().unwrap();

    // Create initial members (user1, user2, user3)
    let _member1 =
        factory::create_guild_member(db, user1.discord_id.parse().unwrap(), guild_id).await?;
    let _member2 =
        factory::create_guild_member(db, user2.discord_id.parse().unwrap(), guild_id).await?;
    let _member3 =
        factory::create_guild_member(db, user3.discord_id.parse().unwrap(), guild_id).await?;

    // Sync to (user2, user3, user4) - keep user2 and user3, remove user1, add user4
    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo
        .sync_guild_members(
            guild_id,
            &[
                (user2.discord_id.parse().unwrap(), "User2".to_string(), None),
                (user3.discord_id.parse().unwrap(), "User3".to_string(), None),
                (user4.discord_id.parse().unwrap(), "User4".to_string(), None),
            ],
        )
        .await;

    assert!(result.is_ok());

    // Verify final state
    let members = repo.get_members_by_guild(guild_id).await?;
    assert_eq!(members.len(), 3);

    let user_ids: Vec<u64> = members.iter().map(|m| m.user_id).collect();
    assert!(!user_ids.contains(&user1.discord_id.parse::<u64>().unwrap()));
    assert!(user_ids.contains(&user2.discord_id.parse::<u64>().unwrap()));
    assert!(user_ids.contains(&user3.discord_id.parse::<u64>().unwrap()));
    assert!(user_ids.contains(&user4.discord_id.parse::<u64>().unwrap()));

    Ok(())
}

/// Tests syncing updates member data.
///
/// Verifies that syncing updates username and nickname for existing members.
///
/// Expected: Ok with member data updated
#[tokio::test]
async fn syncs_updates_member_data() -> Result<(), DbErr> {
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
    let user_id = user.discord_id.parse().unwrap();

    // Create initial member
    let _member =
        factory::create_guild_member_with_nickname(db, user_id, guild_id, "OldNick").await?;

    // Sync with updated data
    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo
        .sync_guild_members(
            guild_id,
            &[(
                user_id,
                "NewUsername".to_string(),
                Some("NewNick".to_string()),
            )],
        )
        .await;

    assert!(result.is_ok());

    // Verify member data is updated
    let member = repo.get_member(user_id, guild_id).await?.unwrap();
    assert_eq!(member.username, "NewUsername");
    assert_eq!(member.nickname, Some("NewNick".to_string()));

    Ok(())
}

/// Tests syncing is idempotent.
///
/// Verifies that calling sync_guild_members multiple times with the same
/// data results in the same final state.
///
/// Expected: Ok with same state after multiple syncs
#[tokio::test]
async fn idempotent_sync() -> Result<(), DbErr> {
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

    let members = vec![
        (user1.discord_id.parse().unwrap(), "User1".to_string(), None),
        (user2.discord_id.parse().unwrap(), "User2".to_string(), None),
    ];

    let repo = DiscordGuildMemberRepository::new(db);

    // Sync three times with same data
    let result1 = repo.sync_guild_members(guild_id, &members).await;
    assert!(result1.is_ok());

    let result2 = repo.sync_guild_members(guild_id, &members).await;
    assert!(result2.is_ok());

    let result3 = repo.sync_guild_members(guild_id, &members).await;
    assert!(result3.is_ok());

    // Verify final state is correct
    let count = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild.guild_id))
        .count(db)
        .await?;
    assert_eq!(count, 2);

    Ok(())
}

/// Tests syncing with many members.
///
/// Verifies that syncing works correctly with a large number of members.
///
/// Expected: Ok with all members synced
#[tokio::test]
async fn syncs_many_members() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::User)
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildMember)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse().unwrap();

    // Create 30 users
    let mut members = Vec::new();
    for _ in 0..30 {
        let user = factory::create_user(db).await?;
        members.push((
            user.discord_id.parse::<u64>().unwrap(),
            format!("User {}", user.discord_id),
            None,
        ));
    }

    let repo = DiscordGuildMemberRepository::new(db);
    let result = repo.sync_guild_members(guild_id, &members).await;

    assert!(result.is_ok());

    // Verify all members exist
    let count = entity::prelude::DiscordGuildMember::find()
        .filter(entity::discord_guild_member::Column::GuildId.eq(&guild.guild_id))
        .count(db)
        .await?;
    assert_eq!(count, 30);

    Ok(())
}

/// Tests syncing with nonexistent guild fails.
///
/// Verifies that attempting to sync members for a guild that doesn't
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
        .sync_guild_members(
            999999999,
            &[(user.discord_id.parse().unwrap(), "User".to_string(), None)],
        )
        .await;

    assert!(result.is_err());

    Ok(())
}

/// Tests syncing with nonexistent user succeeds.
///
/// Verifies that syncing with users that don't exist in the User table
/// succeeds, as guild members track ALL Discord users, not just those
/// with application accounts.
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
        .sync_guild_members(
            guild.guild_id.parse().unwrap(),
            &[(999999999, "User".to_string(), None)],
        )
        .await;

    assert!(result.is_ok());

    // Verify member was created
    let members = repo
        .get_members_by_guild(guild.guild_id.parse().unwrap())
        .await?;
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].user_id, 999999999);

    Ok(())
}

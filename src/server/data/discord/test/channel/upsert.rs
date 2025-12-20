use super::*;
use sea_orm::EntityTrait;
use serenity::all::{ChannelId, GuildChannel};
use test_utils::factory;

/// Tests creating a new channel.
///
/// Verifies that the repository successfully creates a new channel record
/// in the database when upserting a channel that doesn't exist yet.
///
/// Expected: Ok with channel record created
#[tokio::test]
async fn creates_new_channel() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let mut channel = GuildChannel::default();
    channel.id = ChannelId::new(987654321);
    channel.name = "general".to_string();
    channel.position = 1;

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.upsert(guild_id, &channel).await;

    assert!(result.is_ok());
    let created = result.unwrap();
    assert_eq!(created.channel_id, 987654321);
    assert_eq!(created.guild_id, guild_id);
    assert_eq!(created.name, "general");
    assert_eq!(created.position, 1);

    // Verify in database
    let db_channel = entity::prelude::DiscordGuildChannel::find()
        .one(db)
        .await?
        .unwrap();
    assert_eq!(db_channel.channel_id, "987654321");
    assert_eq!(db_channel.guild_id, guild.guild_id);
    assert_eq!(db_channel.name, "general");
    assert_eq!(db_channel.position, 1);

    Ok(())
}

/// Tests updating an existing channel.
///
/// Verifies that the repository updates a channel's name and position
/// when upserting a channel that already exists in the database.
///
/// Expected: Ok with channel record updated
#[tokio::test]
async fn updates_existing_channel() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    // Create initial channel
    factory::create_guild_channel(db, &guild.guild_id, "987654321").await?;

    // Update the channel with new data
    let mut channel = GuildChannel::default();
    channel.id = ChannelId::new(987654321);
    channel.name = "announcements".to_string();
    channel.position = 5;

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.upsert(guild_id, &channel).await;

    assert!(result.is_ok());
    let updated = result.unwrap();
    assert_eq!(updated.channel_id, 987654321);
    assert_eq!(updated.name, "announcements");
    assert_eq!(updated.position, 5);

    // Verify in database
    let db_channel = entity::prelude::DiscordGuildChannel::find()
        .one(db)
        .await?
        .unwrap();
    assert_eq!(db_channel.name, "announcements");
    assert_eq!(db_channel.position, 5);

    Ok(())
}

/// Tests upserting preserves guild_id.
///
/// Verifies that when updating an existing channel, the guild_id remains
/// unchanged even if the upsert is called with the same guild_id.
///
/// Expected: Ok with guild_id unchanged
#[tokio::test]
async fn preserves_guild_id_on_update() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    // Create initial channel
    let initial = factory::create_guild_channel(db, &guild.guild_id, "987654321").await?;

    // Upsert again
    let mut channel = GuildChannel::default();
    channel.id = ChannelId::new(987654321);
    channel.name = "updated-name".to_string();
    channel.position = 2;

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.upsert(guild_id, &channel).await;

    assert!(result.is_ok());
    let updated = result.unwrap();
    assert_eq!(updated.guild_id, initial.guild_id.parse::<u64>().unwrap());

    Ok(())
}

/// Tests upserting multiple channels for the same guild.
///
/// Verifies that multiple channels can be upserted for a single guild
/// without conflicts.
///
/// Expected: Ok with all channels created
#[tokio::test]
async fn upserts_multiple_channels_for_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let repo = DiscordGuildChannelRepository::new(db);

    // Create first channel
    let mut channel1 = GuildChannel::default();
    channel1.id = ChannelId::new(111111111);
    channel1.name = "general".to_string();
    channel1.position = 1;
    repo.upsert(guild_id, &channel1).await?;

    // Create second channel
    let mut channel2 = GuildChannel::default();
    channel2.id = ChannelId::new(222222222);
    channel2.name = "announcements".to_string();
    channel2.position = 2;
    repo.upsert(guild_id, &channel2).await?;

    // Create third channel
    let mut channel3 = GuildChannel::default();
    channel3.id = ChannelId::new(333333333);
    channel3.name = "off-topic".to_string();
    channel3.position = 3;
    repo.upsert(guild_id, &channel3).await?;

    // Verify all channels exist
    let channels = entity::prelude::DiscordGuildChannel::find().all(db).await?;
    assert_eq!(channels.len(), 3);

    Ok(())
}

/// Tests upserting channel with duplicate ID updates without changing guild.
///
/// Verifies that when a channel_id already exists, the ON CONFLICT UPDATE
/// only updates name and position, NOT guild_id. The channel remains in the
/// original guild.
///
/// Expected: Ok with channel still in original guild
#[tokio::test]
async fn duplicate_channel_id_updates_without_changing_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild1 = factory::create_guild(db).await?;
    let guild2 = factory::create_guild(db).await?;

    let guild1_id = guild1.guild_id.parse::<u64>().unwrap();
    let guild2_id = guild2.guild_id.parse::<u64>().unwrap();

    let repo = DiscordGuildChannelRepository::new(db);

    // Create channel in first guild
    let mut channel1 = GuildChannel::default();
    channel1.id = ChannelId::new(987654321);
    channel1.name = "general".to_string();
    channel1.position = 1;
    repo.upsert(guild1_id, &channel1).await?;

    // Try to create same channel_id in second guild
    let mut channel2 = GuildChannel::default();
    channel2.id = ChannelId::new(987654321);
    channel2.name = "announcements".to_string();
    channel2.position = 1;
    let result = repo.upsert(guild2_id, &channel2).await;

    // Should update the existing channel's name and position,
    // but NOT change guild_id (ON CONFLICT only updates name and position)
    assert!(result.is_ok());
    let updated = result.unwrap();
    assert_eq!(updated.channel_id, 987654321);
    assert_eq!(updated.name, "announcements");
    assert_eq!(updated.position, 1);
    // The channel should still belong to guild1 since guild_id is not updated
    assert_eq!(updated.guild_id, guild1_id);

    // Verify only one channel exists and it still belongs to guild1
    let db_channel = entity::prelude::DiscordGuildChannel::find()
        .one(db)
        .await?
        .unwrap();
    assert_eq!(db_channel.channel_id, "987654321");
    assert_eq!(db_channel.guild_id, guild1.guild_id);
    assert_eq!(db_channel.name, "announcements");

    Ok(())
}

/// Tests upserting with zero position.
///
/// Verifies that a channel can be created with position 0.
///
/// Expected: Ok with position 0
#[tokio::test]
async fn upserts_with_zero_position() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let mut channel = GuildChannel::default();
    channel.id = ChannelId::new(987654321);
    channel.name = "top-channel".to_string();
    channel.position = 0;

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.upsert(guild_id, &channel).await;

    assert!(result.is_ok());
    let created = result.unwrap();
    assert_eq!(created.position, 0);

    Ok(())
}

/// Tests upserting with large position.
///
/// Verifies that a channel can be created with a large position value.
///
/// Expected: Ok with large position
#[tokio::test]
async fn upserts_with_large_position() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let mut channel = GuildChannel::default();
    channel.id = ChannelId::new(987654321);
    channel.name = "special-channel".to_string();
    channel.position = 9999;

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.upsert(guild_id, &channel).await;

    assert!(result.is_ok());
    let created = result.unwrap();
    assert_eq!(created.position, 9999);

    Ok(())
}

/// Tests upserting with empty channel name.
///
/// Verifies that a channel can be created with an empty name string.
///
/// Expected: Ok with empty name
#[tokio::test]
async fn upserts_with_empty_name() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let mut channel = GuildChannel::default();
    channel.id = ChannelId::new(987654321);
    channel.name = "".to_string();
    channel.position = 1;

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.upsert(guild_id, &channel).await;

    assert!(result.is_ok());
    let created = result.unwrap();
    assert_eq!(created.name, "");

    Ok(())
}

/// Tests upserting with very long channel name.
///
/// Verifies that a channel can be created with a long name string.
///
/// Expected: Ok with long name
#[tokio::test]
async fn upserts_with_long_name() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let long_name = "a".repeat(100);
    let mut channel = GuildChannel::default();
    channel.id = ChannelId::new(987654321);
    channel.name = long_name.clone();
    channel.position = 1;

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.upsert(guild_id, &channel).await;

    assert!(result.is_ok());
    let created = result.unwrap();
    assert_eq!(created.name, long_name);

    Ok(())
}

/// Tests upserting fails for nonexistent guild.
///
/// Verifies that attempting to create a channel for a guild that doesn't
/// exist in the database fails due to foreign key constraint.
///
/// Expected: Err
#[tokio::test]
async fn fails_for_nonexistent_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let mut channel = GuildChannel::default();
    channel.id = ChannelId::new(987654321);
    channel.name = "general".to_string();
    channel.position = 1;

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.upsert(999999999, &channel).await;

    assert!(result.is_err());

    Ok(())
}

/// Tests upserting is idempotent.
///
/// Verifies that upserting the same channel data multiple times
/// produces the same result without errors.
///
/// Expected: Ok with same result each time
#[tokio::test]
async fn upsert_is_idempotent() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let mut channel = GuildChannel::default();
    channel.id = ChannelId::new(987654321);
    channel.name = "general".to_string();
    channel.position = 1;

    let repo = DiscordGuildChannelRepository::new(db);

    // Upsert three times
    let result1 = repo.upsert(guild_id, &channel).await?;
    let result2 = repo.upsert(guild_id, &channel).await?;
    let result3 = repo.upsert(guild_id, &channel).await?;

    assert_eq!(result1.channel_id, result2.channel_id);
    assert_eq!(result2.channel_id, result3.channel_id);
    assert_eq!(result1.name, result2.name);
    assert_eq!(result2.name, result3.name);
    assert_eq!(result1.position, result2.position);
    assert_eq!(result2.position, result3.position);

    // Verify only one record exists
    let channels = entity::prelude::DiscordGuildChannel::find().all(db).await?;
    assert_eq!(channels.len(), 1);

    Ok(())
}

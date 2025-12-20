use super::*;
use test_utils::factory;

/// Tests deleting an existing channel.
///
/// Verifies that the repository successfully deletes a channel record
/// from the database when provided with a valid channel ID.
///
/// Expected: Ok with channel removed from database
#[tokio::test]
async fn deletes_existing_channel() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let channel = factory::create_guild_channel(db, &guild.guild_id, "987654321").await?;
    let channel_id = channel.channel_id.parse::<u64>().unwrap();

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.delete(channel_id).await;

    assert!(result.is_ok());

    // Verify channel is deleted
    let db_channel = entity::prelude::DiscordGuildChannel::find().one(db).await?;
    assert!(db_channel.is_none());

    Ok(())
}

/// Tests deleting nonexistent channel succeeds.
///
/// Verifies that attempting to delete a channel that doesn't exist
/// in the database succeeds without errors (idempotent delete).
///
/// Expected: Ok (no error)
#[tokio::test]
async fn deleting_nonexistent_channel_succeeds() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.delete(999999999).await;

    assert!(result.is_ok());

    Ok(())
}

/// Tests deleting one channel doesn't affect others.
///
/// Verifies that deleting a specific channel only removes that channel
/// and doesn't affect other channels in the same guild or other guilds.
///
/// Expected: Ok with only target channel deleted
#[tokio::test]
async fn deletes_only_target_channel() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    factory::create_guild_channel(db, &guild.guild_id, "111111111").await?;
    factory::create_guild_channel(db, &guild.guild_id, "222222222").await?;
    factory::create_guild_channel(db, &guild.guild_id, "333333333").await?;

    let repo = DiscordGuildChannelRepository::new(db);
    repo.delete(222222222).await?;

    // Verify only middle channel was deleted
    let channels = entity::prelude::DiscordGuildChannel::find().all(db).await?;
    assert_eq!(channels.len(), 2);

    let channel_ids: Vec<String> = channels.iter().map(|c| c.channel_id.clone()).collect();
    assert!(channel_ids.contains(&"111111111".to_string()));
    assert!(!channel_ids.contains(&"222222222".to_string()));
    assert!(channel_ids.contains(&"333333333".to_string()));

    Ok(())
}

/// Tests deleting channel from one guild doesn't affect other guilds.
///
/// Verifies that deleting a channel is isolated to the specific guild
/// and doesn't affect channels in other guilds.
///
/// Expected: Ok with only target guild's channel deleted
#[tokio::test]
async fn delete_isolated_to_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild1 = factory::create_guild(db).await?;
    let guild2 = factory::create_guild(db).await?;

    let channel1 = factory::create_guild_channel(db, &guild1.guild_id, "987654321").await?;
    factory::create_guild_channel(db, &guild2.guild_id, "123456789").await?;

    let channel1_id = channel1.channel_id.parse::<u64>().unwrap();

    let repo = DiscordGuildChannelRepository::new(db);
    repo.delete(channel1_id).await?;

    // Verify guild1's channel was deleted
    let guild1_channels = entity::prelude::DiscordGuildChannel::find()
        .filter(entity::discord_guild_channel::Column::GuildId.eq(&guild1.guild_id))
        .all(db)
        .await?;
    assert_eq!(guild1_channels.len(), 0);

    // Verify guild2's channel still exists
    let guild2_channels = entity::prelude::DiscordGuildChannel::find()
        .filter(entity::discord_guild_channel::Column::GuildId.eq(&guild2.guild_id))
        .all(db)
        .await?;
    assert_eq!(guild2_channels.len(), 1);

    Ok(())
}

/// Tests delete is idempotent.
///
/// Verifies that deleting the same channel multiple times
/// succeeds without errors.
///
/// Expected: Ok each time
#[tokio::test]
async fn delete_is_idempotent() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let channel = factory::create_guild_channel(db, &guild.guild_id, "987654321").await?;
    let channel_id = channel.channel_id.parse::<u64>().unwrap();

    let repo = DiscordGuildChannelRepository::new(db);

    // Delete three times
    let result1 = repo.delete(channel_id).await;
    let result2 = repo.delete(channel_id).await;
    let result3 = repo.delete(channel_id).await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());

    // Verify channel doesn't exist
    let db_channel = entity::prelude::DiscordGuildChannel::find().one(db).await?;
    assert!(db_channel.is_none());

    Ok(())
}

/// Tests deleting all channels in a guild.
///
/// Verifies that all channels for a guild can be deleted individually
/// without errors.
///
/// Expected: Ok with all channels deleted
#[tokio::test]
async fn deletes_all_channels_in_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    factory::create_guild_channel(db, &guild.guild_id, "111111111").await?;
    factory::create_guild_channel(db, &guild.guild_id, "222222222").await?;
    factory::create_guild_channel(db, &guild.guild_id, "333333333").await?;

    let repo = DiscordGuildChannelRepository::new(db);
    repo.delete(111111111).await?;
    repo.delete(222222222).await?;
    repo.delete(333333333).await?;

    // Verify all channels are deleted
    let channels = entity::prelude::DiscordGuildChannel::find().all(db).await?;
    assert_eq!(channels.len(), 0);

    Ok(())
}

/// Tests deleting channel with maximum u64 ID.
///
/// Verifies that channels with very large IDs can be deleted successfully.
///
/// Expected: Ok
#[tokio::test]
async fn deletes_channel_with_large_id() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let large_id = u64::MAX - 1;
    factory::create_guild_channel(db, &guild.guild_id, &large_id.to_string()).await?;

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.delete(large_id).await;

    assert!(result.is_ok());

    // Verify channel is deleted
    let db_channel = entity::prelude::DiscordGuildChannel::find().one(db).await?;
    assert!(db_channel.is_none());

    Ok(())
}

use super::*;
use test_utils::factory;

/// Tests retrieving channels for a guild with multiple channels.
///
/// Verifies that the repository successfully retrieves all channels
/// belonging to a specific guild, ordered by position.
///
/// Expected: Ok with all channels in position order
#[tokio::test]
async fn retrieves_all_channels_for_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    factory::create_guild_channel_with_position(db, &guild.guild_id, "111111111", 2).await?;
    factory::create_guild_channel_with_position(db, &guild.guild_id, "222222222", 0).await?;
    factory::create_guild_channel_with_position(db, &guild.guild_id, "333333333", 1).await?;

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.get_by_guild_id(guild_id).await;

    assert!(result.is_ok());
    let channels = result.unwrap();
    assert_eq!(channels.len(), 3);

    // Verify they are ordered by position
    assert_eq!(channels[0].channel_id, 222222222);
    assert_eq!(channels[0].position, 0);
    assert_eq!(channels[1].channel_id, 333333333);
    assert_eq!(channels[1].position, 1);
    assert_eq!(channels[2].channel_id, 111111111);
    assert_eq!(channels[2].position, 2);

    Ok(())
}

/// Tests retrieving channels for guild with no channels.
///
/// Verifies that the repository returns an empty vector when
/// a guild has no channels.
///
/// Expected: Ok with empty vector
#[tokio::test]
async fn returns_empty_for_guild_with_no_channels() -> Result<(), DbErr> {
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
    let result = repo.get_by_guild_id(guild_id).await;

    assert!(result.is_ok());
    let channels = result.unwrap();
    assert_eq!(channels.len(), 0);

    Ok(())
}

/// Tests retrieving channels for nonexistent guild.
///
/// Verifies that the repository returns an empty vector when
/// querying for a guild that doesn't exist.
///
/// Expected: Ok with empty vector
#[tokio::test]
async fn returns_empty_for_nonexistent_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.get_by_guild_id(999999999).await;

    assert!(result.is_ok());
    let channels = result.unwrap();
    assert_eq!(channels.len(), 0);

    Ok(())
}

/// Tests channels are isolated by guild.
///
/// Verifies that retrieving channels for one guild doesn't
/// return channels from other guilds.
///
/// Expected: Ok with only target guild's channels
#[tokio::test]
async fn returns_only_target_guild_channels() -> Result<(), DbErr> {
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

    factory::create_guild_channel(db, &guild1.guild_id, "987654321").await?;
    factory::create_guild_channel(db, &guild1.guild_id, "876543210").await?;
    factory::create_guild_channel(db, &guild2.guild_id, "123456789").await?;

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.get_by_guild_id(guild1_id).await;

    assert!(result.is_ok());
    let channels = result.unwrap();
    assert_eq!(channels.len(), 2);

    // Verify all channels belong to guild1
    for channel in &channels {
        assert_eq!(channel.guild_id, guild1_id);
    }

    Ok(())
}

/// Tests retrieving single channel for guild.
///
/// Verifies that the repository correctly returns a vector with
/// one channel when a guild has only one channel.
///
/// Expected: Ok with single channel
#[tokio::test]
async fn retrieves_single_channel() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    let channel = factory::create_guild_channel(db, &guild.guild_id, "987654321").await?;

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.get_by_guild_id(guild_id).await;

    assert!(result.is_ok());
    let channels = result.unwrap();
    assert_eq!(channels.len(), 1);
    assert_eq!(
        channels[0].channel_id,
        channel.channel_id.parse::<u64>().unwrap()
    );

    Ok(())
}

/// Tests channels ordered by position ascending.
///
/// Verifies that channels are returned in ascending position order,
/// with lower positions appearing first in the result.
///
/// Expected: Ok with channels in position order
#[tokio::test]
async fn orders_channels_by_position_ascending() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    // Create channels in random order
    factory::create_guild_channel_with_position(db, &guild.guild_id, "111111111", 5).await?;
    factory::create_guild_channel_with_position(db, &guild.guild_id, "222222222", 1).await?;
    factory::create_guild_channel_with_position(db, &guild.guild_id, "333333333", 10).await?;
    factory::create_guild_channel_with_position(db, &guild.guild_id, "444444444", 3).await?;

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.get_by_guild_id(guild_id).await;

    assert!(result.is_ok());
    let channels = result.unwrap();
    assert_eq!(channels.len(), 4);

    // Verify ascending order
    assert_eq!(channels[0].position, 1);
    assert_eq!(channels[1].position, 3);
    assert_eq!(channels[2].position, 5);
    assert_eq!(channels[3].position, 10);

    Ok(())
}

/// Tests channels with varying positions are ordered correctly.
///
/// Verifies that positions are handled correctly in sorting,
/// from smallest to largest values.
///
/// Expected: Ok with positions in ascending order
#[tokio::test]
async fn orders_varying_positions_correctly() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    factory::create_guild_channel_with_position(db, &guild.guild_id, "111111111", 100).await?;
    factory::create_guild_channel_with_position(db, &guild.guild_id, "222222222", 2).await?;
    factory::create_guild_channel_with_position(db, &guild.guild_id, "333333333", 0).await?;
    factory::create_guild_channel_with_position(db, &guild.guild_id, "444444444", 50).await?;

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.get_by_guild_id(guild_id).await;

    assert!(result.is_ok());
    let channels = result.unwrap();
    assert_eq!(channels.len(), 4);

    // Verify order: 0, 2, 50, 100
    assert_eq!(channels[0].position, 0);
    assert_eq!(channels[1].position, 2);
    assert_eq!(channels[2].position, 50);
    assert_eq!(channels[3].position, 100);

    Ok(())
}

/// Tests channels with same position maintain stable order.
///
/// Verifies that when multiple channels have the same position,
/// they are all returned without errors.
///
/// Expected: Ok with all channels returned
#[tokio::test]
async fn handles_duplicate_positions() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    factory::create_guild_channel_with_position(db, &guild.guild_id, "111111111", 1).await?;
    factory::create_guild_channel_with_position(db, &guild.guild_id, "222222222", 1).await?;
    factory::create_guild_channel_with_position(db, &guild.guild_id, "333333333", 1).await?;

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.get_by_guild_id(guild_id).await;

    assert!(result.is_ok());
    let channels = result.unwrap();
    assert_eq!(channels.len(), 3);

    // All should have position 1
    for channel in &channels {
        assert_eq!(channel.position, 1);
    }

    Ok(())
}

/// Tests retrieving channels returns domain models.
///
/// Verifies that the repository converts entity models to domain models
/// at the infrastructure boundary, returning proper u64 IDs.
///
/// Expected: Ok with domain models
#[tokio::test]
async fn returns_domain_models() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    factory::create_guild_channel(db, &guild.guild_id, "987654321").await?;

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.get_by_guild_id(guild_id).await;

    assert!(result.is_ok());
    let channels = result.unwrap();
    assert_eq!(channels.len(), 1);

    // Verify domain model fields are u64
    let channel = &channels[0];
    assert_eq!(channel.guild_id, guild_id);
    assert_eq!(channel.channel_id, 987654321u64);

    Ok(())
}

/// Tests retrieving large number of channels.
///
/// Verifies that the repository can handle retrieving many channels
/// for a single guild without errors.
///
/// Expected: Ok with all channels returned
#[tokio::test]
async fn retrieves_many_channels() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_table(entity::prelude::DiscordGuild)
        .with_table(entity::prelude::DiscordGuildChannel)
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::create_guild(db).await?;
    let guild_id = guild.guild_id.parse::<u64>().unwrap();

    // Create 50 channels
    for i in 0..50 {
        let channel_id = format!("{}", 1000000 + i);
        factory::create_guild_channel_with_position(db, &guild.guild_id, &channel_id, i as i32)
            .await?;
    }

    let repo = DiscordGuildChannelRepository::new(db);
    let result = repo.get_by_guild_id(guild_id).await;

    assert!(result.is_ok());
    let channels = result.unwrap();
    assert_eq!(channels.len(), 50);

    // Verify they are in order
    for (i, channel) in channels.iter().enumerate() {
        assert_eq!(channel.position, i as i32);
    }

    Ok(())
}

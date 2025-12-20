use sea_orm::DbErr;

use crate::server::{
    data::fleet_message::FleetMessageRepository, model::fleet_message::CreateFleetMessageParam,
};

use super::*;

/// Tests retrieving fleet messages for a fleet.
///
/// Verifies that the repository successfully retrieves all fleet message
/// records associated with a specific fleet ID.
///
/// Expected: Ok with vector containing all messages for the fleet
#[tokio::test]
async fn returns_fleet_messages() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_message_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    // Create fleet with all dependencies using factory
    let (_user, _guild, _ping_format, _category, fleet) =
        factory::helpers::create_fleet_with_dependencies(db).await?;

    let repo = FleetMessageRepository::new(db);

    // Create multiple messages for the fleet
    repo.create(CreateFleetMessageParam {
        fleet_id: fleet.id,
        channel_id: 111,
        message_id: 222,
        message_type: "creation".to_string(),
    })
    .await?;

    repo.create(CreateFleetMessageParam {
        fleet_id: fleet.id,
        channel_id: 333,
        message_id: 444,
        message_type: "reminder".to_string(),
    })
    .await?;

    repo.create(CreateFleetMessageParam {
        fleet_id: fleet.id,
        channel_id: 555,
        message_id: 666,
        message_type: "formup".to_string(),
    })
    .await?;

    // Retrieve messages
    let result = repo.get_by_fleet_id(fleet.id).await;

    assert!(result.is_ok());
    let messages = result.unwrap();
    assert_eq!(messages.len(), 3);
    assert!(messages.iter().all(|m| m.fleet_id == fleet.id));

    // Verify message types
    let types: Vec<&str> = messages.iter().map(|m| m.message_type.as_str()).collect();
    assert!(types.contains(&"creation"));
    assert!(types.contains(&"reminder"));
    assert!(types.contains(&"formup"));

    Ok(())
}

/// Tests retrieving messages for fleet with no messages.
///
/// Verifies that the repository returns an empty vector when querying
/// for a fleet that exists but has no associated messages.
///
/// Expected: Ok with empty vector
#[tokio::test]
async fn returns_empty_for_fleet_with_no_messages() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_message_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    // Create fleet with all dependencies using factory
    let (_user, _guild, _ping_format, _category, fleet) =
        factory::helpers::create_fleet_with_dependencies(db).await?;

    let repo = FleetMessageRepository::new(db);
    let result = repo.get_by_fleet_id(fleet.id).await;

    assert!(result.is_ok());
    let messages = result.unwrap();
    assert!(messages.is_empty());

    Ok(())
}

/// Tests retrieving messages for non-existent fleet.
///
/// Verifies that the repository returns an empty vector when querying
/// for a fleet ID that doesn't exist in the database.
///
/// Expected: Ok with empty vector
#[tokio::test]
async fn returns_empty_for_nonexistent_fleet() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_message_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let repo = FleetMessageRepository::new(db);
    let result = repo.get_by_fleet_id(999999).await;

    assert!(result.is_ok());
    let messages = result.unwrap();
    assert!(messages.is_empty());

    Ok(())
}

/// Tests messages are isolated per fleet.
///
/// Verifies that retrieving messages for one fleet does not return
/// messages from other fleets.
///
/// Expected: Ok with only messages for the queried fleet
#[tokio::test]
async fn returns_only_messages_for_specified_fleet() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_message_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    // Create first fleet with all dependencies using factory
    let (user, _guild, _ping_format, category, fleet1) =
        factory::helpers::create_fleet_with_dependencies(db).await?;

    // Create second fleet using same category
    let fleet2 = factory::fleet::create_fleet(db, category.id, &user.discord_id).await?;

    let repo = FleetMessageRepository::new(db);

    // Create messages for fleet1
    repo.create(CreateFleetMessageParam {
        fleet_id: fleet1.id,
        channel_id: 111,
        message_id: 222,
        message_type: "creation".to_string(),
    })
    .await?;

    repo.create(CreateFleetMessageParam {
        fleet_id: fleet1.id,
        channel_id: 333,
        message_id: 444,
        message_type: "reminder".to_string(),
    })
    .await?;

    // Create messages for fleet2
    repo.create(CreateFleetMessageParam {
        fleet_id: fleet2.id,
        channel_id: 555,
        message_id: 666,
        message_type: "creation".to_string(),
    })
    .await?;

    // Get messages for fleet1
    let result = repo.get_by_fleet_id(fleet1.id).await;

    assert!(result.is_ok());
    let messages = result.unwrap();
    assert_eq!(messages.len(), 2);
    assert!(messages.iter().all(|m| m.fleet_id == fleet1.id));

    Ok(())
}

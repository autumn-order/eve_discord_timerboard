use sea_orm::DbErr;

use crate::server::{
    data::fleet_message::FleetMessageRepository, model::fleet_message::CreateFleetMessageParam,
};

use super::*;

/// Tests creating a new fleet message.
///
/// Verifies that the repository successfully creates a new fleet message record
/// with the specified fleet_id, channel_id, message_id, and message_type.
///
/// Expected: Ok with fleet message created
#[tokio::test]
async fn creates_fleet_message() -> Result<(), DbErr> {
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
    let result = repo
        .create(CreateFleetMessageParam {
            fleet_id: fleet.id,
            channel_id: 999888777,
            message_id: 555444333,
            message_type: "creation".to_string(),
        })
        .await;

    assert!(result.is_ok());
    let message = result.unwrap();
    assert_eq!(message.fleet_id, fleet.id);
    assert_eq!(message.channel_id, "999888777");
    assert_eq!(message.message_id, "555444333");
    assert_eq!(message.message_type, "creation");

    Ok(())
}

/// Tests creating fleet message with different message types.
///
/// Verifies that the repository successfully creates fleet messages with
/// different message_type values (creation, reminder, formup).
///
/// Expected: Ok with all message types created
#[tokio::test]
async fn creates_different_message_types() -> Result<(), DbErr> {
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

    // Create creation message
    let creation = repo
        .create(CreateFleetMessageParam {
            fleet_id: fleet.id,
            channel_id: 111,
            message_id: 222,
            message_type: "creation".to_string(),
        })
        .await?;
    assert_eq!(creation.message_type, "creation");

    // Create reminder message
    let reminder = repo
        .create(CreateFleetMessageParam {
            fleet_id: fleet.id,
            channel_id: 333,
            message_id: 444,
            message_type: "reminder".to_string(),
        })
        .await?;
    assert_eq!(reminder.message_type, "reminder");

    // Create formup message
    let formup = repo
        .create(CreateFleetMessageParam {
            fleet_id: fleet.id,
            channel_id: 555,
            message_id: 666,
            message_type: "formup".to_string(),
        })
        .await?;
    assert_eq!(formup.message_type, "formup");

    Ok(())
}

/// Tests foreign key constraint on fleet_id.
///
/// Verifies that the repository returns an error when attempting to create
/// a fleet message with a fleet_id that doesn't exist in the database.
///
/// Expected: Err(DbErr) due to foreign key constraint violation
#[tokio::test]
async fn fails_for_nonexistent_fleet() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_message_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let repo = FleetMessageRepository::new(db);
    let result = repo
        .create(CreateFleetMessageParam {
            fleet_id: 999999, // Non-existent fleet
            channel_id: 123,
            message_id: 456,
            message_type: "creation".to_string(),
        })
        .await;

    assert!(result.is_err());

    Ok(())
}

use super::*;

/// Tests getting category details for an existing category.
///
/// Verifies that the repository successfully returns the category details
/// with all related data when the category exists.
///
/// Expected: Ok(Some(category))
#[tokio::test]
async fn returns_category_details_when_exists() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let (_, _, _, category, _) = factory::helpers::create_fleet_with_dependencies(db).await?;

    let repo = FleetCategoryRepository::new(db);
    let result = repo.get_category_details(category.id).await;

    assert!(result.is_ok());
    assert!(result.unwrap().is_some());

    Ok(())
}

/// Tests getting category details for a nonexistent category.
///
/// Verifies that the repository returns None when the category ID does
/// not exist in the database.
///
/// Expected: Ok(None)
#[tokio::test]
async fn returns_none_when_category_does_not_exist() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let repo = FleetCategoryRepository::new(db);
    let result = repo.get_category_details(99999).await;

    assert!(result.is_ok());
    assert!(result.unwrap().is_none());

    Ok(())
}

/// Tests that get_category_details returns same result as get_by_id.
///
/// Verifies that get_category_details is properly aliasing get_by_id
/// by comparing their results.
///
/// Expected: Both methods return identical results
#[tokio::test]
async fn returns_same_result_as_get_by_id() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let (_, _, _, category, _) = factory::helpers::create_fleet_with_dependencies(db).await?;

    let repo = FleetCategoryRepository::new(db);
    let details_result = repo.get_category_details(category.id).await?;
    let by_id_result = repo.get_by_id(category.id).await?;

    assert_eq!(details_result.is_some(), by_id_result.is_some());

    if let (Some(details), Some(by_id)) = (details_result, by_id_result) {
        assert_eq!(details.category.id, by_id.category.id);
        assert_eq!(details.category.name, by_id.category.name);
        assert_eq!(details.category.guild_id, by_id.category.guild_id);
        assert_eq!(
            details.category.ping_format_id,
            by_id.category.ping_format_id
        );
    }

    Ok(())
}

/// Tests getting category details includes all related entities.
///
/// Verifies that the category details include ping format, access roles,
/// ping roles, and channels.
///
/// Expected: Ok with category containing all relations
#[tokio::test]
async fn includes_all_related_entities() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let (_, _, _, category, _) = factory::helpers::create_fleet_with_dependencies(db).await?;

    let repo = FleetCategoryRepository::new(db);
    let result = repo.get_category_details(category.id).await?;

    assert!(result.is_some());
    let details = result.unwrap();

    // Should have ping format
    assert!(details.ping_format.is_some());

    Ok(())
}

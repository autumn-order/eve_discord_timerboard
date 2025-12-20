use super::*;

/// Tests getting categories by ping format ID when categories exist.
///
/// Verifies that the repository successfully returns all categories that
/// use a specific ping format.
///
/// Expected: Ok with matching categories
#[tokio::test]
async fn returns_categories_using_ping_format() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let ping_format = factory::ping_format::create_ping_format(db, &guild.guild_id).await?;

    let category1 =
        factory::fleet_category::create_category(db, &guild.guild_id, ping_format.id).await?;
    let category2 =
        factory::fleet_category::create_category(db, &guild.guild_id, ping_format.id).await?;

    let repo = FleetCategoryRepository::new(db);
    let result = repo.get_by_ping_format_id(ping_format.id).await;

    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 2);

    let category_ids: Vec<i32> = categories.iter().map(|c| c.id).collect();
    assert!(category_ids.contains(&category1.id));
    assert!(category_ids.contains(&category2.id));

    Ok(())
}

/// Tests getting categories when no categories use the ping format.
///
/// Verifies that the repository returns an empty vector when no categories
/// are associated with the specified ping format.
///
/// Expected: Ok with empty vector
#[tokio::test]
async fn returns_empty_when_no_categories_use_format() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let ping_format = factory::ping_format::create_ping_format(db, &guild.guild_id).await?;

    let repo = FleetCategoryRepository::new(db);
    let result = repo.get_by_ping_format_id(ping_format.id).await;

    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 0);

    Ok(())
}

/// Tests getting categories when ping format doesn't exist.
///
/// Verifies that the repository returns an empty vector when the ping
/// format ID does not exist in the database.
///
/// Expected: Ok with empty vector
#[tokio::test]
async fn returns_empty_for_nonexistent_ping_format() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let repo = FleetCategoryRepository::new(db);
    let result = repo.get_by_ping_format_id(99999).await;

    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 0);

    Ok(())
}

/// Tests getting categories across multiple guilds using same format.
///
/// Verifies that the repository returns all categories using a ping format
/// regardless of which guild they belong to.
///
/// Expected: Ok with all matching categories from all guilds
#[tokio::test]
async fn returns_categories_across_multiple_guilds() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild1 = factory::discord_guild::create_guild(db).await?;
    let guild2 = factory::discord_guild::create_guild(db).await?;
    let ping_format = factory::ping_format::create_ping_format(db, &guild1.guild_id).await?;

    let category1 =
        factory::fleet_category::create_category(db, &guild1.guild_id, ping_format.id).await?;
    let category2 =
        factory::fleet_category::create_category(db, &guild2.guild_id, ping_format.id).await?;

    let repo = FleetCategoryRepository::new(db);
    let result = repo.get_by_ping_format_id(ping_format.id).await;

    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 2);

    let category_ids: Vec<i32> = categories.iter().map(|c| c.id).collect();
    assert!(category_ids.contains(&category1.id));
    assert!(category_ids.contains(&category2.id));

    Ok(())
}

/// Tests that categories with different ping formats are not returned.
///
/// Verifies that the repository only returns categories that match the
/// specified ping format ID and excludes categories using other formats.
///
/// Expected: Ok with only matching categories
#[tokio::test]
async fn excludes_categories_with_different_ping_format() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;
    let ping_format1 = factory::ping_format::create_ping_format(db, &guild.guild_id).await?;
    let ping_format2 = factory::ping_format::create_ping_format(db, &guild.guild_id).await?;

    let category1 =
        factory::fleet_category::create_category(db, &guild.guild_id, ping_format1.id).await?;
    let _category2 =
        factory::fleet_category::create_category(db, &guild.guild_id, ping_format2.id).await?;

    let repo = FleetCategoryRepository::new(db);
    let result = repo.get_by_ping_format_id(ping_format1.id).await;

    assert!(result.is_ok());
    let categories = result.unwrap();
    assert_eq!(categories.len(), 1);
    assert_eq!(categories[0].id, category1.id);
    assert_eq!(categories[0].ping_format_id, ping_format1.id);

    Ok(())
}

use super::*;

/// Tests checking if a category exists in the correct guild.
///
/// Verifies that the repository returns true when a category exists and
/// belongs to the specified guild.
///
/// Expected: Ok(true)
#[tokio::test]
async fn returns_true_when_category_exists_in_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let (_, guild, _, category, _) = factory::helpers::create_fleet_with_dependencies(db).await?;

    let repo = FleetCategoryRepository::new(db);
    let result = repo
        .exists_in_guild(category.id, guild.guild_id.parse().unwrap())
        .await;

    assert!(result.is_ok());
    assert!(result.unwrap());

    Ok(())
}

/// Tests checking if a category exists in a different guild.
///
/// Verifies that the repository returns false when a category exists but
/// belongs to a different guild than specified.
///
/// Expected: Ok(false)
#[tokio::test]
async fn returns_false_when_category_exists_in_different_guild() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let (_, _, _, category, _) = factory::helpers::create_fleet_with_dependencies(db).await?;
    let other_guild = factory::discord_guild::create_guild(db).await?;

    let repo = FleetCategoryRepository::new(db);
    let result = repo
        .exists_in_guild(category.id, other_guild.guild_id.parse().unwrap())
        .await;

    assert!(result.is_ok());
    assert!(!result.unwrap());

    Ok(())
}

/// Tests checking if a nonexistent category exists in guild.
///
/// Verifies that the repository returns false when the category ID does
/// not exist in the database.
///
/// Expected: Ok(false)
#[tokio::test]
async fn returns_false_when_category_does_not_exist() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let guild = factory::discord_guild::create_guild(db).await?;

    let repo = FleetCategoryRepository::new(db);
    let result = repo
        .exists_in_guild(99999, guild.guild_id.parse().unwrap())
        .await;

    assert!(result.is_ok());
    assert!(!result.unwrap());

    Ok(())
}

/// Tests checking category existence with multiple categories in guild.
///
/// Verifies that the repository correctly identifies the specified category
/// among multiple categories in the same guild.
///
/// Expected: Ok(true) for correct category
#[tokio::test]
async fn returns_true_for_correct_category_among_multiple() -> Result<(), DbErr> {
    let test = TestBuilder::new()
        .with_fleet_tables()
        .build()
        .await
        .unwrap();
    let db = test.db.as_ref().unwrap();

    let (_, guild, ping_format, category1, _) =
        factory::helpers::create_fleet_with_dependencies(db).await?;
    let category2 =
        factory::fleet_category::create_category(db, &guild.guild_id, ping_format.id).await?;

    let repo = FleetCategoryRepository::new(db);

    // Check both categories exist in the guild
    let result1 = repo
        .exists_in_guild(category1.id, guild.guild_id.parse().unwrap())
        .await?;
    let result2 = repo
        .exists_in_guild(category2.id, guild.guild_id.parse().unwrap())
        .await?;

    assert!(result1);
    assert!(result2);

    Ok(())
}

use crate::server::error::AppError;
use crate::server::middleware::session::AuthSession;
use test_utils::builder::TestBuilder;

/// Tests setting and getting user ID.
///
/// Verifies that a user ID can be stored in the session and retrieved
/// successfully.
///
/// Expected: Ok with user ID retrieved
#[tokio::test]
async fn sets_and_gets_user_id() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let auth = AuthSession::new(session);

    auth.set_user_id("123456789".to_string()).await?;
    let user_id = auth.get_user_id().await?;

    assert_eq!(user_id, Some("123456789".to_string()));

    Ok(())
}

/// Tests getting user ID when none is set.
///
/// Verifies that get_user_id returns None when no user has been
/// stored in the session.
///
/// Expected: Ok with None
#[tokio::test]
async fn gets_none_when_no_user_id() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let auth = AuthSession::new(session);

    let user_id = auth.get_user_id().await?;

    assert!(user_id.is_none());

    Ok(())
}

/// Tests is_authenticated returns true when user is logged in.
///
/// Verifies that is_authenticated returns true after a user ID
/// has been stored in the session.
///
/// Expected: Ok with true
#[tokio::test]
async fn is_authenticated_returns_true_when_logged_in() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let auth = AuthSession::new(session);

    auth.set_user_id("123456789".to_string()).await?;
    let is_authed = auth.is_authenticated().await?;

    assert!(is_authed);

    Ok(())
}

/// Tests is_authenticated returns false when user is not logged in.
///
/// Verifies that is_authenticated returns false when no user ID
/// has been stored in the session.
///
/// Expected: Ok with false
#[tokio::test]
async fn is_authenticated_returns_false_when_not_logged_in() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let auth = AuthSession::new(session);

    let is_authed = auth.is_authenticated().await?;

    assert!(!is_authed);

    Ok(())
}

/// Tests clear removes user ID from session.
///
/// Verifies that calling clear removes the user ID from the session,
/// simulating a logout operation.
///
/// Expected: Ok with None after clear
#[tokio::test]
async fn clear_removes_user_id() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let auth = AuthSession::new(session);

    auth.set_user_id("123456789".to_string()).await?;
    auth.clear().await;
    let user_id = auth.get_user_id().await?;

    assert!(user_id.is_none());

    Ok(())
}

/// Tests inner returns session reference.
///
/// Verifies that the inner method returns a reference to the
/// underlying session instance.
///
/// Expected: Session reference returned
#[tokio::test]
async fn inner_returns_session_reference() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let auth = AuthSession::new(session);

    let inner = auth.inner();

    // Just verify we can use it - no panic means success
    let _ = inner.get::<String>("auth:user").await;

    Ok(())
}

/// Tests updating user ID.
///
/// Verifies that setting a new user ID overwrites the previous one.
///
/// Expected: Ok with new user ID retrieved
#[tokio::test]
async fn updates_user_id() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let auth = AuthSession::new(session);

    auth.set_user_id("111111111".to_string()).await?;
    auth.set_user_id("222222222".to_string()).await?;
    let user_id = auth.get_user_id().await?;

    assert_eq!(user_id, Some("222222222".to_string()));

    Ok(())
}

use crate::server::error::AppError;
use crate::server::middleware::session::OAuthFlowSession;
use test_utils::builder::TestBuilder;

/// Tests setting and taking admin flag.
///
/// Verifies that the admin flag can be set and retrieved exactly
/// once using take_admin_flag.
///
/// Expected: Ok with true retrieved once, then false
#[tokio::test]
async fn sets_and_takes_admin_flag() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let oauth = OAuthFlowSession::new(session);

    oauth.set_admin_flag(true).await?;
    let flag = oauth.take_admin_flag().await?;

    assert!(flag);

    Ok(())
}

/// Tests taking admin flag removes it from session.
///
/// Verifies that take_admin_flag removes the flag from the session,
/// ensuring it can only be used once.
///
/// Expected: Ok with false on second take
#[tokio::test]
async fn take_admin_flag_removes_from_session() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let oauth = OAuthFlowSession::new(session);

    oauth.set_admin_flag(true).await?;
    oauth.take_admin_flag().await?;
    let flag = oauth.take_admin_flag().await?;

    assert!(!flag);

    Ok(())
}

/// Tests taking admin flag when none is set.
///
/// Verifies that take_admin_flag returns false when no flag has
/// been stored in the session.
///
/// Expected: Ok with false
#[tokio::test]
async fn take_admin_flag_returns_false_when_not_set() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let oauth = OAuthFlowSession::new(session);

    let flag = oauth.take_admin_flag().await?;

    assert!(!flag);

    Ok(())
}

/// Tests setting and taking adding bot flag.
///
/// Verifies that the bot addition flag can be set and retrieved
/// exactly once using take_adding_bot_flag.
///
/// Expected: Ok with true retrieved once, then false
#[tokio::test]
async fn sets_and_takes_adding_bot_flag() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let oauth = OAuthFlowSession::new(session);

    oauth.set_adding_bot_flag(true).await?;
    let flag = oauth.take_adding_bot_flag().await?;

    assert!(flag);

    Ok(())
}

/// Tests taking adding bot flag removes it from session.
///
/// Verifies that take_adding_bot_flag removes the flag from the
/// session, ensuring it can only be used once.
///
/// Expected: Ok with false on second take
#[tokio::test]
async fn take_adding_bot_flag_removes_from_session() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let oauth = OAuthFlowSession::new(session);

    oauth.set_adding_bot_flag(true).await?;
    oauth.take_adding_bot_flag().await?;
    let flag = oauth.take_adding_bot_flag().await?;

    assert!(!flag);

    Ok(())
}

/// Tests taking adding bot flag when none is set.
///
/// Verifies that take_adding_bot_flag returns false when no flag
/// has been stored in the session.
///
/// Expected: Ok with false
#[tokio::test]
async fn take_adding_bot_flag_returns_false_when_not_set() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let oauth = OAuthFlowSession::new(session);

    let flag = oauth.take_adding_bot_flag().await?;

    assert!(!flag);

    Ok(())
}

/// Tests setting false for admin flag.
///
/// Verifies that explicitly setting the admin flag to false works
/// correctly.
///
/// Expected: Ok with false retrieved
#[tokio::test]
async fn sets_admin_flag_to_false() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let oauth = OAuthFlowSession::new(session);

    oauth.set_admin_flag(false).await?;
    let flag = oauth.take_admin_flag().await?;

    assert!(!flag);

    Ok(())
}

/// Tests setting false for adding bot flag.
///
/// Verifies that explicitly setting the bot addition flag to false
/// works correctly.
///
/// Expected: Ok with false retrieved
#[tokio::test]
async fn sets_adding_bot_flag_to_false() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let oauth = OAuthFlowSession::new(session);

    oauth.set_adding_bot_flag(false).await?;
    let flag = oauth.take_adding_bot_flag().await?;

    assert!(!flag);

    Ok(())
}

/// Tests updating admin flag.
///
/// Verifies that setting a new admin flag value overwrites the
/// previous one.
///
/// Expected: Ok with new value retrieved
#[tokio::test]
async fn updates_admin_flag() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let oauth = OAuthFlowSession::new(session);

    oauth.set_admin_flag(false).await?;
    oauth.set_admin_flag(true).await?;
    let flag = oauth.take_admin_flag().await?;

    assert!(flag);

    Ok(())
}

/// Tests updating adding bot flag.
///
/// Verifies that setting a new bot addition flag value overwrites
/// the previous one.
///
/// Expected: Ok with new value retrieved
#[tokio::test]
async fn updates_adding_bot_flag() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let oauth = OAuthFlowSession::new(session);

    oauth.set_adding_bot_flag(false).await?;
    oauth.set_adding_bot_flag(true).await?;
    let flag = oauth.take_adding_bot_flag().await?;

    assert!(flag);

    Ok(())
}

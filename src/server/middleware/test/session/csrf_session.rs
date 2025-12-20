use crate::server::error::AppError;
use crate::server::middleware::session::CsrfSession;
use test_utils::builder::TestBuilder;

/// Tests setting and taking CSRF token.
///
/// Verifies that a CSRF token can be stored in the session and
/// retrieved exactly once using take_token.
///
/// Expected: Ok with token retrieved once, then None
#[tokio::test]
async fn sets_and_takes_token() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let csrf = CsrfSession::new(session);

    csrf.set_token("test_csrf_token".to_string()).await?;
    let token = csrf.take_token().await?;

    assert_eq!(token, Some("test_csrf_token".to_string()));

    Ok(())
}

/// Tests taking token removes it from session.
///
/// Verifies that take_token removes the token from the session,
/// preventing replay attacks by ensuring each token can only be
/// used once.
///
/// Expected: Ok with None on second take
#[tokio::test]
async fn take_token_removes_from_session() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let csrf = CsrfSession::new(session);

    csrf.set_token("test_csrf_token".to_string()).await?;
    csrf.take_token().await?;
    let token = csrf.take_token().await?;

    assert!(token.is_none());

    Ok(())
}

/// Tests taking token when none is set.
///
/// Verifies that take_token returns None when no token has been
/// stored in the session.
///
/// Expected: Ok with None
#[tokio::test]
async fn take_token_returns_none_when_no_token() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let csrf = CsrfSession::new(session);

    let token = csrf.take_token().await?;

    assert!(token.is_none());

    Ok(())
}

/// Tests updating CSRF token.
///
/// Verifies that setting a new token overwrites the previous one.
///
/// Expected: Ok with new token retrieved
#[tokio::test]
async fn updates_token() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();
    let csrf = CsrfSession::new(session);

    csrf.set_token("old_token".to_string()).await?;
    csrf.set_token("new_token".to_string()).await?;
    let token = csrf.take_token().await?;

    assert_eq!(token, Some("new_token".to_string()));

    Ok(())
}

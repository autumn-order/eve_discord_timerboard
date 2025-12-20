use crate::server::error::AppError;
use crate::server::middleware::session::{AuthSession, CsrfSession, OAuthFlowSession};
use test_utils::builder::TestBuilder;

/// Tests all session types work with same session.
///
/// Verifies that AuthSession, CsrfSession, and OAuthFlowSession can
/// all wrap the same session and operate independently.
///
/// Expected: Ok with all values set and retrieved correctly
#[tokio::test]
async fn all_session_types_work_together() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();

    let auth = AuthSession::new(session);
    let csrf = CsrfSession::new(session);
    let oauth = OAuthFlowSession::new(session);

    // Set values in all session types
    auth.set_user_id("123456789".to_string()).await?;
    csrf.set_token("csrf_token".to_string()).await?;
    oauth.set_admin_flag(true).await?;
    oauth.set_adding_bot_flag(true).await?;

    // Retrieve and verify all values
    assert_eq!(auth.get_user_id().await?, Some("123456789".to_string()));
    assert_eq!(csrf.take_token().await?, Some("csrf_token".to_string()));
    assert!(oauth.take_admin_flag().await?);
    assert!(oauth.take_adding_bot_flag().await?);

    Ok(())
}

/// Tests auth clear removes all session data.
///
/// Verifies that calling clear on AuthSession removes data from
/// all session types (auth, csrf, oauth flow data).
///
/// Expected: Ok with all values cleared
#[tokio::test]
async fn auth_clear_removes_all_data() -> Result<(), AppError> {
    let mut test = TestBuilder::new().build().await.unwrap();
    let session = test.session().await.unwrap();

    let auth = AuthSession::new(session);
    let csrf = CsrfSession::new(session);
    let oauth = OAuthFlowSession::new(session);

    // Set values in all session types
    auth.set_user_id("123456789".to_string()).await?;
    csrf.set_token("csrf_token".to_string()).await?;
    oauth.set_admin_flag(true).await?;

    // Clear session
    auth.clear().await;

    // Verify all values are cleared
    assert!(auth.get_user_id().await?.is_none());
    assert!(csrf.take_token().await?.is_none());
    assert!(!oauth.take_admin_flag().await?);

    Ok(())
}

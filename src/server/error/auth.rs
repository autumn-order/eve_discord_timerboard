use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use oauth2::{basic::BasicErrorResponseType, HttpClientError, StandardErrorResponse};
use thiserror::Error;

use crate::{model::api::ErrorDto, server::error::InternalServerError};

#[derive(Error, Debug)]
pub enum AuthError {
    /// CSRF state validation failed during OAuth callback.
    ///
    /// The CSRF state token in the OAuth callback URL does not match the token stored
    /// in the session, indicating a potential CSRF attack or an invalid callback request.
    /// Results in a 400 Bad Request response.
    #[error("Failed to login user due to CSRF state mismatch")]
    CsrfValidationFailed,
    /// Admin code validation failed.
    ///
    /// The provided admin code is invalid, expired, or does not match the stored code.
    /// Results in a 403 Forbidden response.
    #[error("Invalid or expired admin code")]
    AdminCodeValidationFailed,
    #[error("User not found in session")]
    UserNotInSession,
    #[error("User {0} not found in database")]
    UserNotInDatabase(u64),
    #[error("Access denied for user {0}: {1}")]
    AccessDenied(u64, String),
    #[error(transparent)]
    RequestTokenErr(
        #[from]
        oauth2::RequestTokenError<
            HttpClientError<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),
}

/// Converts authentication errors into HTTP responses.
///
/// Maps authentication errors to appropriate HTTP status codes and user-friendly error messages:
/// - `UserNotInSession` / `UserNotInDatabase` → 404 Not Found with "User not found"
/// - `CsrfValidationFailed` / `CsrfMissingValue` → 400 Bad Request with "There was an issue logging you in"
/// - `CharacterOwnedByAnotherUser` / `CharacterNotOwned` → 400 Bad Request with "Invalid character selection"
/// - Other errors → 500 Internal Server Error with generic message
///
/// All errors are logged at debug level for diagnostics while keeping client-facing messages
/// generic to avoid information leakage.
///
/// # Returns
/// - 400 Bad Request - For CSRF failures and invalid character operations
/// - 404 Not Found - For missing users
/// - 500 Internal Server Error - For unexpected authentication errors
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let user_not_found = (
            StatusCode::NOT_FOUND,
            Json(ErrorDto {
                error: "User not found".to_string(),
            }),
        )
            .into_response();

        match self {
            Self::CsrfValidationFailed => (
                StatusCode::BAD_REQUEST,
                Json(ErrorDto {
                    error: "There was an issue logging you in, please try again.".to_string(),
                }),
            )
                .into_response(),
            Self::UserNotInSession => user_not_found,
            Self::UserNotInDatabase(_) => user_not_found,
            Self::AdminCodeValidationFailed => (
                StatusCode::FORBIDDEN,
                Json(ErrorDto {
                    error: "Invalid or expired admin code.".to_string(),
                }),
            )
                .into_response(),
            Self::AccessDenied(_, _) => (
                StatusCode::FORBIDDEN,
                Json(ErrorDto {
                    error: "Insufficient permissions".to_string(),
                }),
            )
                .into_response(),
            err => InternalServerError(err).into_response(),
        }
    }
}

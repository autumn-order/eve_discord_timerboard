//! HTTP request helpers with automatic rate limit handling.
//!
//! This module provides utility functions for making HTTP requests to the backend API
//! with automatic retry logic for rate-limited requests (429 responses).
//!
//! # Rate Limit Handling
//!
//! The `send_request()` function automatically handles 429 (Too Many Requests) responses
//! by retrying with intelligent delay logic. It accepts a closure that builds the request,
//! allowing the same request to be recreated for each retry attempt.
//!
//! ## Usage Examples
//!
//! ```ignore
//! // Simple GET request
//! pub async fn get_users() -> Result<Vec<UserDto>, ApiError> {
//!     let response = send_request(|| get("/api/users")).await?;
//!     parse_response(response).await
//! }
//!
//! // POST with body
//! pub async fn create_user(dto: CreateUserDto) -> Result<UserDto, ApiError> {
//!     let body = serialize_json(&dto)?;
//!     let response = send_request(|| post("/api/users").body(body.clone())).await?;
//!     parse_response(response).await
//! }
//!
//! // DELETE request
//! pub async fn delete_user(user_id: u64) -> Result<(), ApiError> {
//!     let url = format!("/api/users/{}", user_id);
//!     let response = send_request(|| delete(&url)).await?;
//!     parse_empty_response(response).await
//! }
//! ```
//!
//! The retry logic will:
//! - Attempt the request up to 3 times (initial + 3 retries = 4 total attempts)
//! - Respect the server's `Retry-After` header when present
//! - Fall back to exponential backoff (1s, 2s, 4s) if header is missing or invalid
//! - Log warnings on each retry attempt with the delay being used
//! - Return a clear error message if all retries are exhausted

use crate::{client::model::error::ApiError, model::api::ErrorDto};
use dioxus_logger::tracing;
use gloo_timers::future::sleep;
use reqwasm::http::{Request, Response};
use serde::de::DeserializeOwned;
use std::time::Duration;

/// Maximum number of retry attempts for rate-limited requests
const MAX_RETRIES: u32 = 3;

/// Initial delay in milliseconds for exponential backoff
const INITIAL_RETRY_DELAY_MS: u64 = 1000;

/// Helper function to parse API responses with consistent error handling
pub async fn parse_response<T: DeserializeOwned>(response: Response) -> Result<T, ApiError> {
    let status = response.status() as u64;

    if (200..300).contains(&status) {
        response.json::<T>().await.map_err(|e| ApiError {
            status: 500,
            message: format!("Failed to parse response: {}", e),
        })
    } else {
        let message = if let Ok(error_dto) = response.json::<ErrorDto>().await {
            error_dto.error
        } else {
            response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string())
        };

        Err(ApiError { status, message })
    }
}

/// Helper function to parse empty success responses (204 No Content, 201 Created, etc.)
pub async fn parse_empty_response(response: Response) -> Result<(), ApiError> {
    let status = response.status() as u64;

    if (200..300).contains(&status) {
        Ok(())
    } else {
        let message = if let Ok(error_dto) = response.json::<ErrorDto>().await {
            error_dto.error
        } else {
            response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string())
        };

        Err(ApiError { status, message })
    }
}

/// Create a GET request with credentials
pub fn get(url: &str) -> Request {
    Request::get(url).credentials(reqwasm::http::RequestCredentials::Include)
}

/// Create a POST request with credentials and JSON content type
pub fn post(url: &str) -> Request {
    Request::post(url)
        .credentials(reqwasm::http::RequestCredentials::Include)
        .header("Content-Type", "application/json")
}

/// Create a PUT request with credentials and JSON content type
pub fn put(url: &str) -> Request {
    Request::put(url)
        .credentials(reqwasm::http::RequestCredentials::Include)
        .header("Content-Type", "application/json")
}

/// Create a DELETE request with credentials
pub fn delete(url: &str) -> Request {
    Request::delete(url).credentials(reqwasm::http::RequestCredentials::Include)
}

/// Send a request with automatic retry for rate limits (429 responses).
///
/// Automatically retries requests that receive 429 (Too Many Requests) responses
/// using exponential backoff. Will attempt up to MAX_RETRIES times before failing.
///
/// # Arguments
/// - `request_builder` - A closure that creates a new Request for each attempt
///
/// # Returns
/// - `Ok(Response)` - Successful response
/// - `Err(ApiError)` - Request failed after all retries or non-429 error occurred
///
/// # Example
/// ```ignore
/// let response = send_request(|| get("/api/users")).await?;
/// ```
pub async fn send_request<F>(request_builder: F) -> Result<Response, ApiError>
where
    F: Fn() -> Request,
{
    let mut attempt = 0;

    loop {
        let request = request_builder();
        let response = request.send().await.map_err(|e| ApiError {
            status: 500,
            message: format!("Failed to send request: {}", e),
        })?;

        // Check if we got rate limited
        if response.status() == 429 {
            if attempt < MAX_RETRIES {
                // Try to get Retry-After header from response
                let delay_ms = if let Some(retry_after_str) = response.headers().get("Retry-After")
                {
                    // Parse Retry-After header (can be seconds or HTTP date)
                    // Try parsing as seconds first
                    if let Ok(seconds) = retry_after_str.parse::<u64>() {
                        let delay = seconds * 1000; // Convert to milliseconds
                        tracing::debug!(
                            "Using Retry-After header value: {}s ({}ms)",
                            seconds,
                            delay
                        );
                        delay
                    } else {
                        // Fall back to exponential backoff if not a number
                        // (HTTP date format parsing is complex and not worth implementing)
                        let delay = INITIAL_RETRY_DELAY_MS * 2u64.pow(attempt);
                        tracing::debug!(
                            "Retry-After header not numeric (value: '{}'), using exponential backoff: {}ms",
                            retry_after_str,
                            delay
                        );
                        delay
                    }
                } else {
                    // No Retry-After header, use exponential backoff
                    let delay = INITIAL_RETRY_DELAY_MS * 2u64.pow(attempt);
                    tracing::debug!(
                        "No Retry-After header, using exponential backoff: {}ms",
                        delay
                    );
                    delay
                };

                tracing::warn!(
                    "Rate limited (429), retrying in {}ms (attempt {}/{})",
                    delay_ms,
                    attempt + 1,
                    MAX_RETRIES
                );

                // Wait before retrying
                sleep(Duration::from_millis(delay_ms)).await;
                attempt += 1;
                continue;
            } else {
                tracing::error!(
                    "Rate limited (429) after {} retries, giving up",
                    MAX_RETRIES
                );

                return Err(ApiError {
                    status: 429,
                    message: format!(
                        "Too many requests. Please try again later. (Failed after {} retries)",
                        MAX_RETRIES
                    ),
                });
            }
        }

        return Ok(response);
    }
}

/// Serialize a payload to JSON string
pub fn serialize_json<T: serde::Serialize>(payload: &T) -> Result<String, ApiError> {
    serde_json::to_string(payload).map_err(|e| ApiError {
        status: 500,
        message: format!("Failed to serialize request: {}", e),
    })
}

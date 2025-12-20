//! Mock objects for external dependencies.
//!
//! This module provides mock implementations and builders for external library types
//! used in testing. Unlike fixtures (which create internal entity models) and factories
//! (which persist entity models to the database), mocks simulate external dependencies
//! such as third-party APIs and library types.
//!
//! # Overview
//!
//! Mocks are used to simulate external systems without making real API calls or requiring
//! external services to be running. This allows for isolated, fast, and reliable tests.
//!
//! # Available Mocks
//!
//! - `serenity` - Mock Discord API objects (Guild, Role, etc.) from the Serenity crate
//!
//! # Usage
//!
//! ```rust,ignore
//! use test_utils::mock::serenity::{create_test_guild, create_test_role};
//!
//! #[tokio::test]
//! async fn test_discord_integration() {
//!     // Create mock Discord objects
//!     let guild = create_test_guild(123456789, "Test Guild", Some("abc123"));
//!     let role = create_test_role(111111111, "Admin", 0xFF0000, 10);
//!
//!     // Use in your tests...
//! }
//! ```
//!
//! # Mock vs Fixture vs Factory
//!
//! | Type | Purpose | Data Type | Database |
//! |------|---------|-----------|----------|
//! | **Mock** | Simulate external APIs | 3rd party types | ❌ No |
//! | **Fixture** | Create test entities | `entity::*::Model` | ❌ No |
//! | **Factory** | Create & persist entities | `entity::*::Model` | ✅ Yes |

pub mod serenity;

// Re-export commonly used functions for convenience
pub use serenity::{create_test_guild, create_test_role};

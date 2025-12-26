//! Fleet notification service for Discord message management.
//!
//! This module provides the `FleetNotificationService` for managing Discord notifications
//! related to fleet events. It orchestrates fleet message posting, updates, and cancellations
//! across configured Discord channels with role pings and embedded fleet information.
//!
//! The service is organized into separate modules by concern:
//! - `builder` - Embed building utilities
//! - `creation` - Initial fleet creation notifications
//! - `reminder` - Fleet reminder notifications
//! - `formup` - Fleet formup (start) notifications
//! - `list` - Upcoming fleets list management

pub mod builder;
pub mod cancel;
pub mod creation;
pub mod formup;
pub mod list;
pub mod reminder;
pub mod update;

use sea_orm::DatabaseConnection;
use serenity::http::Http;
use std::sync::Arc;

/// Service providing Discord notification operations for fleet events.
///
/// This struct holds references to the database connection, Discord HTTP client, and
/// application URL. It provides methods for posting fleet notifications (creation,
/// reminders, formup), and maintaining an upcoming fleets list in configured channels.
///
/// The service layer contains business logic and coordinates between repositories
/// (data layer) and the Discord API. It does not perform direct database queries or
/// entity conversions - those responsibilities belong to the repository layer.
pub struct FleetNotificationService<'a> {
    /// Database connection for accessing fleet and notification data via repositories
    db: &'a DatabaseConnection,
    /// Discord HTTP client for sending and editing messages
    http: Arc<Http>,
    /// Base application URL for embedding links in notifications
    app_url: String,
}

impl<'a> FleetNotificationService<'a> {
    /// Creates a new FleetNotificationService instance.
    ///
    /// # Arguments
    /// - `db` - Reference to the database connection
    /// - `http` - Arc-wrapped Discord HTTP client for API requests
    /// - `app_url` - Base URL of the application for embedding in notifications
    ///
    /// # Returns
    /// - `FleetNotificationService` - New service instance
    pub fn new(db: &'a DatabaseConnection, http: Arc<Http>, app_url: String) -> Self {
        Self { db, http, app_url }
    }
}

//! Automated scheduler for periodic tasks and notifications.
//!
//! This module provides cron-based scheduling for automated tasks that run at
//! regular intervals throughout the application lifecycle. Schedulers are initialized
//! during server startup and continue running until shutdown.
//!
//! Current schedulers include:
//! - Fleet notifications - Sends automated Discord notifications for fleet reminders,
//!   form-ups, and maintains upcoming fleets lists in configured channels

pub mod fleet_notifications;

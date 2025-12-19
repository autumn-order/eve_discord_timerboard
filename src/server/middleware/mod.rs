//! Middleware for request processing and authorization.
//!
//! This module provides middleware components for handling cross-cutting concerns
//! in the application, including authentication, authorization, and session management.
//! The primary middleware is the `AuthGuard`, which enforces permission-based access
//! control across API endpoints.

pub mod auth;
pub mod session;

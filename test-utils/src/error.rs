use thiserror::Error;

/// Test error type for test setup and execution failures.
///
/// These errors occur during test context initialization, typically when setting up
/// the in-memory database or creating tables. Test errors are generally fatal to the
/// specific test case and will cause the test to fail immediately.
#[derive(Error, Debug)]
pub enum TestError {
    /// Database operation failed during test setup.
    ///
    /// Wraps any `sea_orm::DbErr` that occurs when connecting to the in-memory SQLite
    /// database, creating tables, or executing setup queries. Common causes include
    /// invalid SQL statements in table creation or constraint violations.
    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),
}

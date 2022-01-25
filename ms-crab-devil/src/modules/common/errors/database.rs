use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("database connection failed")]
    ConnectionFailed,
    #[error("connection locking failed")]
    ConnectionLockingFailed,
    #[error("diesel error-{0}")]
    InnerDatabaseError(String),
    #[error("repository error-{0}")]
    RepositoryError(String),
}

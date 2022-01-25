use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenericError {
    #[error("bad request")]
    BadRequest,
    #[error("invalid uuid")]
    InvalidUuid,
    #[error("missing environment variable: {0}")]
    MissingEnvironmentVariable(String),
}

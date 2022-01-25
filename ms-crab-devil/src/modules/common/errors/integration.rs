use thiserror::Error;

#[derive(Error, Debug)]
pub enum IntegrationError {
    #[error("S3Error::{0}")]
    S3(S3Error),
}

#[derive(Error, Debug)]
pub enum S3Error {
    #[error("object not found {0}/{1}")]
    ObjectNotFound(String, String),
    #[error("object not found {0}/{1}")]
    FailedToPutObject(String, String),
}

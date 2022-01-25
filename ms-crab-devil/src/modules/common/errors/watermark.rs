use thiserror::Error;

#[derive(Error, Debug)]
pub enum WatermarkError {
    #[error("failed to load document")]
    FailedToLoadDocument,
    #[error("other {0}")]
    Other(String),
}

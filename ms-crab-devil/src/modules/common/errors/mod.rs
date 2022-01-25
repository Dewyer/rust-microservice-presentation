use crate::modules::common::errors::generic::GenericError;
use thiserror::Error;
use tonic::Status;

pub mod database;
pub mod generic;
pub mod integration;
pub mod watermark;

use crate::modules::common::errors::database::DatabaseError;
use crate::modules::common::errors::integration::{IntegrationError, S3Error};
use crate::modules::common::errors::watermark::WatermarkError;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Generic::{0}")]
    Generic(GenericError),
    #[error("Database::{0}")]
    Database(DatabaseError),
    #[error("Watermark::{0}")]
    Watermark(WatermarkError),
    #[error("Integration::{0}")]
    Integration(IntegrationError),
}

impl std::convert::From<diesel::result::Error> for ApiError {
    fn from(diesel_err: diesel::result::Error) -> Self {
        ApiError::Database(DatabaseError::InnerDatabaseError(format!("{}", diesel_err)))
    }
}

impl std::convert::From<DatabaseError> for ApiError {
    fn from(error: DatabaseError) -> Self {
        ApiError::Database(error)
    }
}


impl std::convert::From<S3Error> for ApiError {
    fn from(error: S3Error) -> Self {
        ApiError::Integration(IntegrationError::S3(error))
    }
}

impl std::convert::From<ApiError> for Status {
    fn from(error: ApiError) -> Self {
        Status::unknown(format!("{}", error))
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

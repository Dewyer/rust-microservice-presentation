use crate::modules::common::errors::generic::GenericError;
use uuid::Uuid;

pub struct UuidUtils {}

impl UuidUtils {
    pub fn parse_uuid(id_str: &str) -> Result<Uuid, GenericError> {
        Uuid::parse_str(id_str)
            .map(|v| v)
            .map_err(|_| GenericError::InvalidUuid)
    }

    pub fn parse_optional_uuid(id_str: &Option<String>) -> Result<Option<Uuid>, GenericError> {
        if let Some(id_some_str) = id_str.as_ref() {
            Self::parse_uuid(id_some_str).map(|v| Some(v))
        } else {
            Ok(None)
        }
    }
}

use crate::modules::common::config::ConfigProviderImpl;
use tonic::metadata::MetadataValue;
use tonic::{Request, Status};

pub fn authentication_interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    let api_key = ConfigProviderImpl::get_api_token_unchecked();
    let token = MetadataValue::from_str(&format!("Bearer {}", api_key)).unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("invalid auth token")),
    }
}

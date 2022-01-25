use crate::modules::common::errors::generic::GenericError;
use koval::{Container, Injectable, ResolutionResult};
use std::sync::Arc;

pub trait ConfigProviderTrait: Send + Sync {
    fn check_configuration(&self) -> Result<(), GenericError>;

    fn get_api_token(&self) -> String;

    fn get_output_bucket(&self) -> String;

    fn get_database_url(&self) -> String;
}

pub type IConfigProvider = Arc<dyn ConfigProviderTrait>;

#[derive(Clone)]
pub struct ConfigProviderImpl {}

const DATABASE_URL_VAR_NAME: &'static str = "DATABASE_URL";
const API_TOKE_VAR_NAME: &'static str = "API_TOKEN";

const OUTPUT_BUCKET: &'static str = "OUTPUT_BUCKET";

const REQUIRED_CONFIG_KEYS: [&'static str; 3] =
    [DATABASE_URL_VAR_NAME, API_TOKE_VAR_NAME, OUTPUT_BUCKET];

impl ConfigProviderImpl {
    pub fn get_env_var_or_err(name: &str) -> Result<String, GenericError> {
        std::env::var(name).map_err(|_| GenericError::MissingEnvironmentVariable(name.to_string()))
    }
}

impl ConfigProviderTrait for ConfigProviderImpl {
    fn check_configuration(&self) -> Result<(), GenericError> {
        for var_name in REQUIRED_CONFIG_KEYS.iter() {
            Self::get_env_var_or_err(var_name)?;
        }

        Ok(())
    }

    fn get_api_token(&self) -> String {
        Self::get_env_var_or_err(API_TOKE_VAR_NAME).unwrap()
    }

    fn get_output_bucket(&self) -> String {
        Self::get_env_var_or_err(OUTPUT_BUCKET).unwrap()
    }

    fn get_database_url(&self) -> String {
        Self::get_env_var_or_err(DATABASE_URL_VAR_NAME).unwrap()
    }
}

impl ConfigProviderImpl {
    pub fn get_api_token_unchecked() -> String {
        Self::get_env_var_or_err(API_TOKE_VAR_NAME).unwrap()
    }
}

impl Injectable<IConfigProvider> for ConfigProviderImpl {
    fn resolve_injectable(_: &Container) -> ResolutionResult<IConfigProvider> {
        Ok(Arc::new(ConfigProviderImpl {}))
    }
}

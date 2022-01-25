use super::super::entity::watermarking_log::{NewWatermarkingLog, WatermarkingLog};
use crate::crud_repository;
use crate::schema::watermarking_log;
use diesel::prelude::*;
use koval::{Container, Injectable, InjectionError};
use std::sync::Arc;

crud_repository!(
    WatermarkingLogCrudRepositoryTrait,
    WatermarkingLogCrudRepository,
    watermarking_log,
    WatermarkingLog,
    NewWatermarkingLog,
    "WatermarkingLog"
);

pub type IWatermarkingLogCrudRepository = Arc<dyn WatermarkingLogCrudRepositoryTrait>;

pub trait WatermarkingLogRepositoryTrait: Send + Sync {
    fn crud_repository(&self) -> &IWatermarkingLogCrudRepository;
}

pub type IWatermarkingLogRepository = Arc<dyn WatermarkingLogRepositoryTrait>;

pub struct WatermarkingLogRepository {
    crud_repository: IWatermarkingLogCrudRepository,
}

impl WatermarkingLogRepository {
    pub fn new() -> Self {
        Self {
            crud_repository: Arc::new(WatermarkingLogCrudRepository::new()),
        }
    }
}

impl WatermarkingLogRepositoryTrait for WatermarkingLogRepository {
    fn crud_repository(&self) -> &IWatermarkingLogCrudRepository {
        &self.crud_repository
    }
}

impl Injectable<IWatermarkingLogRepository> for WatermarkingLogRepository {
    fn resolve_injectable(_: &Container) -> Result<IWatermarkingLogRepository, InjectionError> {
        Ok(Arc::new(WatermarkingLogRepository::new()))
    }
}

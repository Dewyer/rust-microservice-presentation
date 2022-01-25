use crate::modules::watermark::domain::service::watermark_service::{
    IWatermarkService, WatermarkServiceImpl,
};
use crate::modules::watermark::infrastructure::repository::watermarking_log_repository::{
    IWatermarkingLogRepository, WatermarkingLogRepository,
};
use koval::Container;

pub fn create_watermark_module() -> Container {
    Container::new()
        .bind_singleton::<IWatermarkService, WatermarkServiceImpl>()
        .bind_singleton::<IWatermarkingLogRepository, WatermarkingLogRepository>()
}

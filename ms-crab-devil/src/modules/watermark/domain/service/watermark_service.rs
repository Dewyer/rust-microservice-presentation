use super::super::super::interface::{WatermarkPdfRequest, WatermarkPdfResponse};
use crate::modules::common::config::IConfigProvider;
use crate::modules::common::errors::watermark::WatermarkError;
use crate::modules::common::errors::{ApiError, ApiResult};
use crate::modules::common::infrastructure::aws::s3_service::S3Service;
use crate::modules::common::infrastructure::transaction::ITransactionManager;
use crate::modules::common::utils::TimeUtils;
use crate::modules::watermark::domain::service::pdf_modification_service::{
    PdfModificationService, WatermarkPdfPayload,
};
use crate::modules::watermark::infrastructure::entity::watermarking_log::NewWatermarkingLog;
use crate::modules::watermark::infrastructure::repository::watermarking_log_repository::IWatermarkingLogRepository;
use koval::{Container, Injectable, ResolutionResult};
use rusoto_s3::StreamingBody;
use std::sync::Arc;
use uuid::Uuid;

#[tonic::async_trait]
pub trait WatermarkServiceTrait: Send + Sync {
    async fn watermark_pdf(&self, request: WatermarkPdfRequest) -> ApiResult<WatermarkPdfResponse>;
}

pub type IWatermarkService = Arc<dyn WatermarkServiceTrait>;

pub struct WatermarkServiceImpl {
    transaction_manager: ITransactionManager,
    config_provider: IConfigProvider,
    watermarking_log_repository: IWatermarkingLogRepository,
}

#[tonic::async_trait]
impl WatermarkServiceTrait for WatermarkServiceImpl {
    async fn watermark_pdf(&self, request: WatermarkPdfRequest) -> ApiResult<WatermarkPdfResponse> {
        let gotten_object = S3Service::get_object(&request.file_bucket, &request.file_key).await?;

        let pdf_result =
            PdfModificationService::watermark_pdf(WatermarkPdfPayload {
                estimated_size: gotten_object.content_length.unwrap_or(0) as usize,
                byte_stream: gotten_object.body.ok_or(ApiError::Watermark(
                    WatermarkError::Other("No body in s3 object".to_string()),
                ))?,
                watermark_text: format!("This is a watermark for {}", request.full_name),
            })
            .await?;

        let streaming_output_body = StreamingBody::from(pdf_result.pdf_bytes);

        let output_bucket = self.config_provider.get_output_bucket();
        let output_file_key = format!("watermarked-w-rust/{}.pdf", Uuid::new_v4().to_string());

        S3Service::put_object(&output_bucket, &output_file_key, streaming_output_body).await?;

        self.transaction_manager.transaction(|tr| {
            self.watermarking_log_repository
                .crud_repository()
                .insert(
                    &NewWatermarkingLog {
                        full_name: &request.full_name,
                        file_key: &request.file_key,
                        file_bucket: &request.file_bucket,
                        watermarked_at: TimeUtils::get_current_utc_time(),
                    },
                    &tr,
                )
                .map_err(|err| err.into())
        })?;

        Ok(WatermarkPdfResponse {
            output_bucket,
            output_file_key,
        })
    }
}

impl Injectable<IWatermarkService> for WatermarkServiceImpl {
    fn resolve_injectable(container: &Container) -> ResolutionResult<IWatermarkService> {
        Ok(Arc::new(WatermarkServiceImpl {
            config_provider: container.resolve()?,
            watermarking_log_repository: container.resolve()?,
            transaction_manager: container.resolve()?,
        }))
    }
}

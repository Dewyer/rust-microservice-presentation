use super::super::interface::{
    watermark_service_server::WatermarkService as WatermarkServiceRpc, WatermarkPdfRequest,
    WatermarkPdfResponse,
};
use crate::modules::watermark::domain::service::watermark_service::IWatermarkService;
use koval::{Container, FromContainer, ResolutionResult};
use tonic::{Request, Response, Status};

pub struct WatermarkServiceHandler {
    watermark_service: IWatermarkService,
}

impl FromContainer for WatermarkServiceHandler {
    fn from_container(container: &Container) -> ResolutionResult<Self> {
        Ok(Self {
            watermark_service: container
                .resolve()
                .expect("Watermark service couldn't be injected"),
        })
    }
}

#[tonic::async_trait]
impl WatermarkServiceRpc for WatermarkServiceHandler {
    async fn watermark_pdf(
        &self,
        request: Request<WatermarkPdfRequest>,
    ) -> Result<Response<WatermarkPdfResponse>, Status> {
        Ok(Response::new(
            self.watermark_service
                .watermark_pdf(request.into_inner())
                .await?,
        ))
    }
}

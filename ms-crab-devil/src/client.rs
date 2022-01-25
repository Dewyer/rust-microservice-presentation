extern crate crab_devil;

use crab_devil::modules::common::infrastructure::aws::s3_service::S3Service;
use crab_devil::modules::watermark::interface::{
    watermark_service_client::WatermarkServiceClient, WatermarkPdfRequest, WatermarkPdfResponse,
};
use std::time::Instant;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::Request;

async fn download() {
    let now = Instant::now();
    let gotten_object = S3Service::get_object(
        "bucket",
        "file",
    )
    .await
    .unwrap();
    println!("LOL {}ms - took", now.elapsed().as_millis());
}

async fn test_watermark_service() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://127.0.0.1:50051")
        .connect()
        .await?;
    let token = MetadataValue::from_str("Bearer AUTH TOKEN HEREl")?;

    let mut client =
        WatermarkServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
            req.metadata_mut().insert("authorization", token.clone());
            Ok(req)
        });

    let request_one = tonic::Request::new(WatermarkPdfRequest {
        file_bucket: "BUCKET HERE".to_string(),
        file_key: "FILE HERE".to_string(),
        full_name: "NAME HERE".to_string(),
    });

    let now = Instant::now();
    let response: tonic::Response<WatermarkPdfResponse> = client.watermark_pdf(request_one).await?;
    println!("{}ms - took", now.elapsed().as_millis());
    println!("PDF_RESPONSE={:?}", response.into_inner());

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    download().await;
    test_watermark_service().await?;

    Ok(())
}

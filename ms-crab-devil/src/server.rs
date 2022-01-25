extern crate crab_devil;

use crab_devil::modules::common::authentication::authentication_interceptor;
use crab_devil::modules::common::config::IConfigProvider;
use crab_devil::modules::common::ioc::full_service_container::create_full_service_container;
use crab_devil::modules::watermark::interface::watermark_service_server::WatermarkServiceServer;
use crab_devil::modules::watermark::watermark_service_handler::WatermarkServiceHandler;
use koval::FromContainer;
use log::info;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    crab_devil::setup();
    let container = create_full_service_container();
    let config = container
        .resolve::<IConfigProvider>()
        .expect("Config provider not injected");
    config.check_configuration().expect("Configuration invalid");
    info!("Configuration checked");

    let watermark_serv = WatermarkServiceServer::with_interceptor(
        WatermarkServiceHandler::from_container(&container)?,
        authentication_interceptor,
    );

    let addr = "127.0.0.1:50051".parse()?;

    info!("Starting server");
    Server::builder()
        .add_service(watermark_serv)
        .serve(addr)
        .await?;

    Ok(())
}

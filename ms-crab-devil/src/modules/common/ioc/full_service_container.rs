use crate::modules::common::common_module::create_common_module;
use crate::modules::common::infrastructure::module::create_common_infrastructure_module;
use crate::modules::watermark::module::create_watermark_module;
use koval::Container;

pub fn create_full_service_container() -> Container {
    Container::new()
        .bind_container_into(create_common_module())
        .bind_container_into(create_common_infrastructure_module())
        .bind_container_into(create_watermark_module())
        .build()
        .expect("Full container failed to build")
}

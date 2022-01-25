use crate::modules::common::config::{ConfigProviderImpl, IConfigProvider};
use koval::Container;

pub fn create_common_module() -> Container {
    Container::new().bind_transient::<IConfigProvider, ConfigProviderImpl>()
}

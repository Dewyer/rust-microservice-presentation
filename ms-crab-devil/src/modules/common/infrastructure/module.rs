use crate::modules::common::infrastructure::database::{
    ConnectionManagerService, IConnectionManagerService,
};
use crate::modules::common::infrastructure::transaction::{
    ITransactionManager, TransactionManager,
};
use koval::Container;

pub fn create_common_infrastructure_module() -> Container {
    Container::new()
        .bind_singleton::<IConnectionManagerService, ConnectionManagerService>()
        .bind_singleton::<ITransactionManager, TransactionManager>()
}

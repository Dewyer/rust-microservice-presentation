use crate::modules::common::errors::{ApiError, ApiResult};
use crate::modules::common::infrastructure::database::IConnectionManagerService;
use crate::modules::common::infrastructure::transaction::{ITransaction, Transaction};
use diesel::Connection;
use koval::{Container, Injectable, InjectionError};
use std::sync::Arc;

pub struct TransactionManager {
    connection_manager_service: IConnectionManagerService,
}

pub type ITransactionManager = Arc<TransactionManager>;

impl TransactionManager {
    pub fn transaction<T, Fn>(&self, fn_in: Fn) -> ApiResult<T>
    where
        Fn: FnOnce(ITransaction) -> ApiResult<T>,
    {
        let conn = Arc::new(
            self.connection_manager_service
                .get_connection_from_pool()
                .map_err(|err| ApiError::Database(err))?,
        );
        conn.transaction(|| fn_in(Box::new(Transaction::new(conn.clone()))))
    }

    pub fn new(connection_manager_service: IConnectionManagerService) -> Self {
        Self {
            connection_manager_service,
        }
    }
}

impl Injectable<ITransactionManager> for TransactionManager {
    fn resolve_injectable(container: &Container) -> Result<ITransactionManager, InjectionError> {
        Ok(Arc::new(Self::new(container.resolve()?)))
    }
}

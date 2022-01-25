use crate::modules::common::infrastructure::database::DbConnection;
use std::sync::Arc;

pub trait ITransactionTrait {
    fn get_db_connection(&self) -> &DbConnection;
}

pub struct Transaction {
    db_connection: Arc<DbConnection>,
}

impl Transaction {
    pub fn new(db_connection: Arc<DbConnection>) -> Self {
        Self { db_connection }
    }
}

impl ITransactionTrait for Transaction {
    fn get_db_connection(&self) -> &DbConnection {
        &self.db_connection
    }
}

pub type ITransaction = Box<dyn ITransactionTrait>;

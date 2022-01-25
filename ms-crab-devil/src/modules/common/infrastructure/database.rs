use crate::modules::common::config::IConfigProvider;
use crate::modules::common::errors::database::DatabaseError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use koval::{Container, Injectable, InjectionError};
use r2d2;
use std::sync::Arc;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub trait ConnectionManagerServiceTrait: Send + Sync {
    fn get_connection_from_pool(&self) -> Result<DbConnection, DatabaseError>;
}

pub struct ConnectionManagerService {
    pool: Pool,
}

pub type IConnectionManagerService = Arc<dyn ConnectionManagerServiceTrait>;

impl ConnectionManagerService {
    pub fn new(config_provider: IConfigProvider) -> Result<Self, InjectionError> {
        info!("Initializing connection pool");

        let db_url = config_provider.get_database_url();
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool = Pool::new(manager).map_err(|_| {
            InjectionError::ResolutionFailed("pool couldn't be created".to_string())
        })?;

        Ok(Self { pool })
    }
}

impl ConnectionManagerServiceTrait for ConnectionManagerService {
    fn get_connection_from_pool(&self) -> Result<DbConnection, DatabaseError> {
        self.pool.get().map_err(|_| DatabaseError::ConnectionFailed)
    }
}

impl Injectable<IConnectionManagerService> for ConnectionManagerService {
    fn resolve_injectable(
        container: &Container,
    ) -> Result<IConnectionManagerService, InjectionError> {
        Ok(Arc::new(Self::new(container.resolve()?)?))
    }
}

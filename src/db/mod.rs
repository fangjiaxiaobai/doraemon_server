pub mod pool;
pub mod config;
pub mod error;
pub mod models;

use crate::db::config::DatabaseConfig;
use crate::db::error::DatabaseError;
use crate::db::pool::DbPool;

pub struct Database {
    pool: DbPool,
}

impl Database {
    pub async fn new(config: &DatabaseConfig) -> Result<Self, DatabaseError> {
        let (url, max, min, timeout) = config.to_pool_config();
        let pool = DbPool::new(&url, max, min, timeout)
            .await
            .map_err(|_e| DatabaseError::PoolInitFailed)?;
        
        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> &DbPool {
        &self.pool
    }
}
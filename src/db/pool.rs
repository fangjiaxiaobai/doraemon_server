use sqlx::{
    Error as SqlxError,
    mysql::{MySqlPool, MySqlPoolOptions},
};
use std::time::Duration;
use std::collections::HashMap;
use crate::db::DatabaseConfig;

#[derive(Debug)]
pub struct DbPool {
    inner: MySqlPool,
}

impl DbPool {
    /// 从配置创建连接池
    pub async fn new(
        url: &str,
        max_connections: u32,
        min_connections: u32,
        acquire_timeout: Duration,
    ) -> Result<Self, SqlxError> {
        let pool = MySqlPoolOptions::new()
            .max_connections(max_connections)
            .min_connections(min_connections)
            .acquire_timeout(acquire_timeout)
            .idle_timeout(Duration::from_secs(300))
            .test_before_acquire(true)
            .connect(url)
            .await?;

        // 定期检查连接健康
        // tokio::spawn(async move {
        //     loop {
        //         let is_healthy = health_check().await;
        //         if !is_healthy {
        //             // 触发报警或重启逻辑
        //         }
        //         tokio::time::sleep(Duration::from_secs(60)).await;
        //     }
        // });

        Ok(Self { inner: pool })
    }

    /// 获取原始连接池引用
    pub fn get_pool(&self) -> &MySqlPool {
        &self.inner
    }

    /// 健康检查
    pub async fn health_check(&self) -> bool {
        match sqlx::query("SELECT 1").execute(&self.inner).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// 当前活跃连接数
    pub fn size(&self) -> u32 {
        self.inner.size()
    }

    /// 空闲连接数
    pub fn num_idle(&self) -> usize {
        self.inner.num_idle()
    }

    pub async fn reload_config(&mut self, config: &DatabaseConfig) -> Result<(), SqlxError> {
        let new_pool = DbPool::new(
            &config.url,
            config.max_connections,
            config.min_connections,
            Duration::from_secs(config.acquire_timeout_secs),
        )
        .await?;

        // todo reload config.
        // self.inner = new_pool.inner;
        Ok(())
    }
}

// 实现Clone trait保证安全克隆
impl Clone for DbPool {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Drop for DbPool {
    fn drop(&mut self) {
        if self.inner.size() > 0 {
            log::warn!("Possible connection leak detected!");
        }
    }
}

// 多租户支持
struct MultiTenantPool {
    pools: HashMap<String, DbPool>,
    config: DatabaseConfig,
}

impl MultiTenantPool {
    async fn get_tenant_pool(&mut self, tenant_id: &str) -> &DbPool {
        if !self.pools.contains_key(tenant_id) {
            let pool = DbPool::new(
                &format!("mysql://{}_{}", tenant_id, self.config.url),
                self.config.max_connections,
                self.config.min_connections,
                Duration::from_secs(self.config.acquire_timeout_secs),
            )
            .await
            .unwrap();
            self.pools.insert(tenant_id.into(), pool);
        }
        self.pools.get(tenant_id).unwrap()
    }
}

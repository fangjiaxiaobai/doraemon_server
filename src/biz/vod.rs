use crate::db::{DbPool, DatabaseError};
use sqlx::mysql::MySqlQueryResult;

pub struct VodQuery;

impl VodQuery {
    /// 创建视频（示例）
    pub async fn create_vod(
        pool: &DbPool,
        name: &str,
        type_id: i32
    ) -> Result<u64, DatabaseError> {
        let result = sqlx::query!(
            "INSERT INTO dor_vod_col (vod_name, type_id) VALUES (?, ?)",
            name,
            type_id
        )
        .execute(pool.get_pool())
        .await?;

        Ok(result.last_insert_id() as u64)
    }

    /// 分页查询（带连接复用）
    pub async fn paginate(
        pool: &DbPool,
        page: u32,
        page_size: u32
    ) -> Result<Vec<Vod>, DatabaseError> {
        let offset = (page - 1) * page_size;
        sqlx::query_as!(
            Vod,
            "SELECT * FROM dor_vod_col LIMIT ? OFFSET ?",
            page_size as i32,
            offset as i32
        )
        .fetch_all(pool.get_pool())
        .await
        .map_err(Into::into)
    }
}

// 步骤5：使用示例
#[cfg(test)]
mod tests {
    use super::*;

    async fn test_pool() -> DbPool {
        let config = DatabaseConfig::default();
        let (url, max, min, timeout) = config.to_pool_config();
        DbPool::new(&url, max, min, timeout)
            .await
            .expect("Failed to create pool")
    }

    #[tokio::test]
    async fn test_pool_management() {
        let pool = test_pool().await;
        
        // 测试连接复用
        let handles: Vec<_> = (0..10).map(|i| {
            let pool = pool.clone();
            tokio::spawn(async move {
                VodQuery::create_vod(&pool, &format!("Video {}", i), 1).await
            })
        }).collect();

        let results = futures::future::join_all(handles).await;
        assert_eq!(results.len(), 10);
    }

    #[tokio::test]
    async fn test_connection_health() {
        let pool = test_pool().await;
        assert!(pool.health_check().await);
    }
}
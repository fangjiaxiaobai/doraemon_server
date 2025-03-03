use thiserror::Error;
use sqlx::Error as SqlxError;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("数据库连接失败: {0}")]
    ConnectionError(#[source] SqlxError),
    
    #[error("查询执行失败: {0}")]
    QueryError(#[source] SqlxError),
    
    #[error("未找到数据")]
    NotFound,
    
    #[error("配置加载失败: {0}")]
    ConfigError(String),
}

impl From<SqlxError> for DatabaseError {
    fn from(err: SqlxError) -> Self {
        DatabaseError::QueryError(err)
    }
}
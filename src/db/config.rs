use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_secs: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "mysql://doraemon:admin.123@localhost:3306/doraemon".into(),
            max_connections: 10,
            min_connections: 2,
            acquire_timeout_secs: 30,
        }
    }
}

impl DatabaseConfig {
    pub fn to_pool_config(&self) -> (String, u32, u32, Duration) {
        (
            self.url.clone(),
            self.max_connections,
            self.min_connections,
            Duration::from_secs(self.acquire_timeout_secs),
        )
    }
}

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Vod {
    pub id: i64,
    pub vod_name: String,
    pub type_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVod {
    pub vod_name: String,
    pub type_id: i32,
}

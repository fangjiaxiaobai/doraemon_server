use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Vod {
    pub id: i64,
    pub vod_col_name: String,
    pub type_id: i32,
    pub vod_pic: String,
    pub vod_actor: String,
    pub vod_director: String,
    pub vod_content: String,
    pub vod_lang: String,
    pub vod_time: NaiveDateTime,
    pub vod_state: String,
    pub vod_remarks: String,
    pub vod_area: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVod {
    pub int: i32,
    pub vod_col_id: i32,
    pub vod_col_name: String,
    pub vod_name: String,
    pub type_id: i32,
    pub vod_play_url: String,
    pub vod_type: String,
    pub vod_player_id: i32,
}

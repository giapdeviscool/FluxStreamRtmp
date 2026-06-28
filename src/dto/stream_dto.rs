use crate::types::stream_enum::StreamStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
//stream dtos
#[derive(Serialize, Deserialize, Debug)]
pub struct StreamResponse {
    pub stream_id: Uuid,
    pub app: String,
    pub stream_name: String,
    pub stream_key: String,
    pub status: StreamStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListStreamResponse {
    pub data: Vec<StreamResponse>,
    pub total: u64,
    pub total_page: u64,
    pub current_page: u64,
}

#[derive(Deserialize, Debug)]
pub struct StreamListQuery {
    pub page: u64,
    pub limit: u64,
}

#[derive(Deserialize, Debug)]
pub struct StreamCreateRequest {
    pub app: String,
    pub stream_name: String,
}

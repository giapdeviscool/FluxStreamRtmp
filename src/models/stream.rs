use crate::types::stream_enum::StreamStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Stream {
    pub id: Uuid,
    pub stream_id: Uuid,
    pub app: String,
    pub stream_name: String,
    pub stream_key: String,
    pub status: StreamStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


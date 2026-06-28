use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum StreamStatus {
    Idle,
    Live,
}

impl StreamStatus {
    pub fn to_string(&self) -> String {
        match self {
            StreamStatus::Idle => "idle".to_string(),
            StreamStatus::Live => "live".to_string(),
        }
    }
}

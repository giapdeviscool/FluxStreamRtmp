use crate::dto::stream_dto::{
    ListStreamResponse, StreamCreateRequest, StreamListQuery, StreamResponse,
};
use crate::errors::server_error::ServerError;
use crate::state::ServerState;
use crate::types::stream_enum::StreamStatus;
use axum::Json;
use axum::extract::{Path, Query, State};
use chrono::Utc;
use uuid::Uuid;

pub async fn list_streams(
    State(_server_state): State<ServerState>,
    Query(query): Query<StreamListQuery>,
) -> Result<Json<ListStreamResponse>, ServerError> {
    let select_page = query.page;
    let limit = query.limit;
    let _offset = (select_page - 1) * limit;

    Ok(Json(ListStreamResponse {
        data: vec![],
        total: 0,
        total_page: 0,
        current_page: 0,
    }))
}

pub async fn get_stream(
    State(_server_state): State<ServerState>,
    Path(stream_id): Path<Uuid>,
) -> Result<Json<StreamResponse>, ServerError> {
    tracing::info!("Get stream: {}", stream_id);
    Ok(Json(StreamResponse {
        stream_id,
        app: "GLIVE".to_string(),
        stream_name: "test".to_string(),
        stream_key: format!("{}:{}", "GLIVE".to_string(), Uuid::now_v7()),
        status: StreamStatus::Idle,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }))
}

pub async fn create_stream(
    State(_server_state): State<ServerState>,
    Json(stream): Json<StreamCreateRequest>,
) -> Result<Json<StreamResponse>, ServerError> {
    let app = stream.app;
    let stream_name = stream.stream_name;
    Ok(Json(StreamResponse {
        stream_id: Uuid::now_v7(),
        app,
        stream_name,
        stream_key: format!("{}:{}", "GLIVE".to_string(), Uuid::now_v7()),
        status: StreamStatus::Idle,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }))
}

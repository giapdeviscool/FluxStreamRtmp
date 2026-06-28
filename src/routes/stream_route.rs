use crate::handlers::stream_handler::*;
use crate::state::ServerState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn route() -> Router<ServerState> {
    Router::new()
        .route("/list", get(list_streams))
        .route("/", post(create_stream))
        .route("/{stream_id}", get(get_stream))
}

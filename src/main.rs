use axum::Router;
use clap::Parser;
use my_rtmp_standalone::routes::stream_route;
use my_rtmp_standalone::state::ServerState;
use my_rtmp_standalone::{config::serverconfig::ServerConfig, core::rtmp::RtmpServer};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use std::fs::File;
use std::io::BufReader;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    config: Option<String>,
}

fn load_config() -> ServerConfig {
    let args = Args::parse();
    let config_path = args
        .config
        .unwrap_or_else(|| "config.development.yaml".to_string());
    let file = File::open(&config_path)
        .unwrap_or_else(|e| panic!("Failed to open config file '{}': {}", config_path, e));
    let reader = BufReader::new(file);
    serde_yaml::from_reader(reader)
        .unwrap_or_else(|e| panic!("Failed to parse config file '{}': {}", config_path, e))
}

async fn build_rtmp(server_state: ServerState) {
    let server = RtmpServer::new(
        server_state.config.server.host.clone(),
        server_state.config.server.rtmp.port,
    );
    if let Err(e) = server.run().await {
        tracing::error!("Failed to start RTMP server: {}", e);
    }
}

async fn build_http(server_state: ServerState) {
    let port = server_state.config.server.http.port;
    let host = &server_state.config.server.host;
    let http_address = format!("{}:{}", host, port);
    let listener = match tokio::net::TcpListener::bind(&http_address).await {
        Ok(listener) => listener,
        Err(e) => {
            tracing::error!("HTTP server error: {}", e);
            return;
        }
    };

    tracing::info!("HTTP Server is running on http://{}:{}", "127.0.0.1", port);
    let app = Router::new()
        .nest("/stream", stream_route::route())
        .with_state(server_state);
    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("HTTP server error: {}", e);
    }
}

async fn build_database_connection(config: &ServerConfig) -> Result<PgPool, sqlx::Error> {
    let database_url = &config.database.url;
    tracing::info!("Connecting to database: {}", database_url);
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await;
    db_pool
}

async fn run_server(server_state: ServerState) {
    tokio::join!(
        build_rtmp(server_state.clone()),
        build_http(server_state.clone())
    );
}

#[tokio::main]
async fn main() {
    // 1. Khởi tạo tracing/logger
    tracing_subscriber::fmt::init();

    // 2. Load config
    let config = load_config();
    let db_pool = match build_database_connection(&config).await {
        Ok(pool) => {
            tracing::info!("Database connection established successfully");
            pool
        }
        Err(e) => {
            tracing::error!("Failed to build database connection: {}", e);
            return;
        }
    };
    let server_state = ServerState::new(config, db_pool);
    run_server(server_state).await;
}

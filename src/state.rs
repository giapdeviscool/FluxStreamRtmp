use sqlx::PgPool;

use crate::config::serverconfig::ServerConfig;

#[derive(Debug, Clone)]
pub struct ServerState {
    pub config: ServerConfig,
    pub db_pool: PgPool,
}

impl ServerState {
    pub fn new(config: ServerConfig, db_pool: PgPool) -> Self {
        Self { config, db_pool }
    }
}

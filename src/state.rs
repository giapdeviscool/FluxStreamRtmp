use crate::config::serverconfig::ServerConfig;

#[derive(Debug, Clone)]
pub struct ServerState {
    pub config: ServerConfig,
}

impl ServerState {
    pub fn new(config: ServerConfig) -> Self {
        Self { config }
    }
}

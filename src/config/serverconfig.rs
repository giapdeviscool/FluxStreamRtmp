use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Rtmp {
    pub port: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Http {
    pub port: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Server {
    pub app: String,
    pub host: String,
    pub rtmp: Rtmp,
    pub http: Http,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Logger {
    pub level: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub server: Server,
    pub database: Database,
    pub logger: Logger,
}

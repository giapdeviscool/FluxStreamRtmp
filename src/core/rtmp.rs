use scuffle_rtmp::{
    ServerSession,
    session::server::{ServerSessionError, SessionData, SessionHandler},
};
use tokio::net::TcpListener;

// =========================================================================
// 1. Handler xử lý sự kiện của từng Session
// =========================================================================
pub struct RtmpHandler;

impl SessionHandler for RtmpHandler {
    async fn on_data(
        &mut self,
        stream_id: u32,
        data: SessionData,
    ) -> Result<(), ServerSessionError> {
        tracing::info!("Received data on stream {}: {:?}", stream_id, data);
        Ok(())
    }

    async fn on_publish(
        &mut self,
        stream_id: u32,
        app_name: &str,
        stream_name: &str,
    ) -> Result<(), ServerSessionError> {
        tracing::info!(
            "Stream {} published with app '{}' and stream name '{}'",
            stream_id,
            app_name,
            stream_name
        );
        Ok(())
    }

    async fn on_unpublish(&mut self, stream_id: u32) -> Result<(), ServerSessionError> {
        Ok(())
    }
}

// =========================================================================
// 2. Server quản lý kết nối TCP và khởi chạy
// =========================================================================
pub struct RtmpServer {
    host: String,
    port: u16,
}

impl RtmpServer {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        let addr = format!("{}:{}", self.host, self.port);
        let listener = TcpListener::bind(&addr).await?;
        tracing::info!("RTMP Server is running on {}", addr);

        while let Ok((stream, addr)) = listener.accept().await {
            tracing::info!("New connection from {}", addr);
            let session = ServerSession::new(stream, RtmpHandler);

            tokio::spawn(async move {
                match session.run().await {
                    Ok(ok) => {
                        tracing::info!("Session ended successfully {}", ok);
                    }
                    Err(err) => {
                        tracing::error!("Session error: {err}");
                    }
                }
            });
        }
        Ok(())
    }
}

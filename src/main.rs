use scuffle_rtmp::{ServerSession, session::server::{ServerSessionError, SessionData, SessionHandler}};
use tokio::net::TcpListener;
use tracing_subscriber;
use axum::Router;
struct Handler;

impl SessionHandler for Handler {
    async fn on_data(&mut self, stream_id: u32, data: SessionData) -> Result<(), ServerSessionError> {
        // Handle incoming video/audio/meta data
        tracing::info!("Received data on stream {}: {:?}", stream_id, data);
        Ok(())
    }

    async fn on_publish(&mut self, stream_id: u32, app_name: &str, stream_name: &str) -> Result<(), ServerSessionError> {
        // Handle the publish event
        tracing::info!("Stream {} published with app '{}' and stream name '{}'", stream_id, app_name, stream_name);
        Ok(())
    }

    async fn on_unpublish(&mut self, stream_id: u32) -> Result<(), ServerSessionError> {
        // Handle the unpublish event
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let listener = TcpListener::bind("0.0.0.0:1999").await.unwrap();
    // listening on 0.0.0.0:1999

    while let Ok((stream, addr)) = listener.accept().await {
        println!("New connection from {}", addr);
        let session = ServerSession::new(stream, Handler);
        tokio::spawn(async move {
            match session.run().await {
                Ok(ok) => {
                    println!("Session ended successfully {}", ok);
                }
                Err(err) => {
                    eprintln!("Session error: {err}");
                }
            }
        });
    }
}
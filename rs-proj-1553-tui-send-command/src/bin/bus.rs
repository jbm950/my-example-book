use rs1553_net::net::tcp_bus;

use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let stdout_layer = fmt::layer()
        .with_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")));

    tracing_subscriber::registry().with(stdout_layer).init();

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Bind listener failed");
    info!("Listening on 127.0.0.1:8080");
    let _ = tcp_bus(listener).await;
}

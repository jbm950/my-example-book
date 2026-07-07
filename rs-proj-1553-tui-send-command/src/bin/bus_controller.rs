use rs1553_net::app::run;

use tokio::io;
use tracing::info;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let stdout_layer = fmt::layer()
        .with_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")));

    tracing_subscriber::registry().with(stdout_layer).init();

    info!("Starting bus controller");
    let server_addr = "127.0.0.1:8080"
        .parse()
        .expect("Socket address parser failed");

    let _ = run(server_addr).await;

    Ok(())
}

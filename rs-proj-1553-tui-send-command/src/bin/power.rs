use rs1553_net::devices::power;

use tracing::{error, info};
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let stdout_layer = fmt::layer()
        .with_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")));

    tracing_subscriber::registry().with(stdout_layer).init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        error!("RT address needs to be passed as a command line arg");
        std::process::exit(1);
    }

    let rt_addr: u8 = args[1].parse().expect("Failed to parse RT address");

    info!("Starting RT {rt_addr}");
    let server_addr = "127.0.0.1:8080"
        .parse()
        .expect("Socket address parser failed");

    let _ = power::run(server_addr, rt_addr).await;
}

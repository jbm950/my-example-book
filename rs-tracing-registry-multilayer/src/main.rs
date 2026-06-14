use std::fs::File;

use tracing::{debug, error, info, level_filters::LevelFilter, trace, warn};
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

const LOG_FILE: &str = "app.log";

fn main() {
    let log_file = File::create(LOG_FILE).expect("Failed to create log file");
    let log_layer = fmt::layer()
        .with_writer(log_file)
        .with_ansi(false) // Turn off ANSI escape characters
        .with_filter(LevelFilter::TRACE);

    // Allow user to set RUST_LOG environment variable to change stdout log
    // level. Note EnvFilter is a feature that has to be enabled from
    // tracing_subscriber
    let stdout_layer = fmt::layer()
        .with_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")));

    tracing_subscriber::registry()
        .with(log_layer)
        .with(stdout_layer)
        .init();

    trace!("This is a TRACE message!");
    debug!("This is a DEBUG message!");
    info!("This is an INFO message!");
    warn!("This is a WARN message!");
    error!("This is an ERROR message!");
}

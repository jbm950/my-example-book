use tracing::{Level, debug, error, info, trace, warn};

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    trace!("This is a TRACE message!");
    debug!("This is a DEBUG message!");
    info!("This is an INFO message!");
    warn!("This is a WARN message!");
    error!("This is an ERROR message!");
}

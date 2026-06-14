use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod module1 {
    use super::*;

    #[tracing::instrument]
    pub fn good_func() {
        trace!("This is a TRACE message!");
        debug!("This is a DEBUG message!");
        info!("This is an INFO message!");
        warn!("This is a WARN message!");
        error!("This is an ERROR message!");
    }
}

mod module2 {
    use super::*;

    #[tracing::instrument]
    pub fn cool_func() {
        trace!("This is a TRACE message!");
        debug!("This is a DEBUG message!");
        info!("This is an INFO message!");
        warn!("This is a WARN message!");
        error!("This is an ERROR message!");
    }

    pub mod submodule2 {
        use super::*;

        #[tracing::instrument]
        pub fn neat_func() {
            trace!("This is a TRACE message!");
            debug!("This is a DEBUG message!");
            info!("This is an INFO message!");
            warn!("This is a WARN message!");
            error!("This is an ERROR message!");
        }
    }
}

fn main() {
    let stdout_layer = fmt::layer()
        .with_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")));

    tracing_subscriber::registry().with(stdout_layer).init();

    module1::good_func();
    module2::cool_func();
    module2::submodule2::neat_func();
}

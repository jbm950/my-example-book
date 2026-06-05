use tracing::{Level, debug, error, info, span, trace, warn};

#[tracing::instrument]
fn inst_func(y: u32) {
    trace!("Using 'y' to do amazing things!");
    debug!("I'm in an instrumented function");
    warn!("Oh my goodness, it's 'y'!");
}

#[tracing::instrument]
fn parent_func() {
    debug!("I'm in a parent function");
    child_func();
}

#[tracing::instrument]
fn child_func() {
    info!("I'm in a child function");
}

#[tracing::instrument(skip(password), fields(user_id = %id))]
fn sensitive_func(id: u32, password: &str) {
    info!("Doing some sensitive work");
}

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    {
        let span_1 = span!(Level::TRACE, "Span 1 Name");
        let _enter = span_1.enter();

        trace!("Trace in Span 1");
        debug!("Debug in Span 1");
        info!("Info in Span 1");
    }

    {
        let x = 5;
        let _span_2 = span!(Level::TRACE, "Span 2 Name", x).entered();

        warn!("Warning in Span 2!");
        error!(x, "Error in Span 2! x too small");
    }

    {
        let _span_3 = span!(Level::TRACE, "Span 3 Name").entered();
        let _child_span = span!(Level::TRACE, "Child Span").entered();

        trace!("Trace in Child Span");
        debug!("Debug in Child Span");
        info!("Info in Child Span");
    }

    inst_func(13);

    parent_func();

    sensitive_func(32, "good_password");
}

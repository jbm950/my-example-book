use tracing::info;

#[tracing::instrument]
pub async fn worker(name: &str) {
    info!("Started");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::fmt;

    #[tokio::test]
    async fn test_worker_completion_times() {
        let _ = fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .try_init();

        let a = tokio::spawn(worker("A"));
        let b = tokio::spawn(worker("B"));
        let c = tokio::spawn(worker("C"));

        let _ = tokio::join!(a, b, c);
    }
}

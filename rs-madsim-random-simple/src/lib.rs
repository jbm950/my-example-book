use rand::RngExt;
use tracing::info;

#[tracing::instrument]
pub async fn worker(name: &str) {
    let delay_ms = {
        let mut rng = rand::rng();
        rng.random_range(100..=1000)
    };
    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
    info!(delay_ms, "Complete");
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

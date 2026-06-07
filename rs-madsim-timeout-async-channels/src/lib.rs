use rand::RngExt;
use tokio::sync::oneshot::{Receiver, Sender};
use tracing::info;

#[tracing::instrument(skip(tx, rx))]
pub async fn server(tx: Sender<String>, rx: Receiver<String>) {
    rx.await.unwrap();

    let delay_ms = {
        let mut rng = rand::rng();
        // On rare occasions, waits up to 50 ms longer than client allows
        rng.random_range(200..=1050)
    };
    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;

    let _ = tx.send(String::from("Response to client"));

    info!(delay_ms, "Server sent response");
}

#[tracing::instrument(skip(tx, rx))]
pub async fn client(tx: Sender<String>, rx: Receiver<String>) {
    let _ = tx.send(String::from("Message to server"));

    let response = tokio::time::timeout(tokio::time::Duration::from_secs(1), rx)
        .await
        .expect("Timed out!");

    info!(?response, "Response received");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::oneshot;
    use tracing_subscriber::fmt;

    #[tokio::test]
    async fn request_completes_before_timeout() {
        let _ = fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .try_init();

        let (client_tx, client_rx) = oneshot::channel();
        let (server_tx, server_rx) = oneshot::channel();

        let a = tokio::spawn(client(client_tx, server_rx));
        let b = tokio::spawn(server(server_tx, client_rx));

        let (client_result, server_result) = tokio::join!(a, b);

        client_result.unwrap();
        server_result.unwrap();
    }
}

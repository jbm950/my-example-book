use std::net::SocketAddr;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::{debug, info};

#[tracing::instrument(skip(listener))]
pub async fn serve(listener: TcpListener) {
    let (mut stream, client_ip) = listener.accept().await.expect("Error accepting connection");
    debug!(?client_ip, "Accepted connection");

    let mut buffer = [0u8; 64];
    let bytes_read = stream.read(&mut buffer).await.expect("server: read failed");
    let msg = String::from_utf8_lossy(&buffer[..bytes_read]);

    debug!(?msg, "Server read the buffer");

    stream.write_all(b"Hello World").await.expect("server: write failed");
    stream.flush().await.expect("server: flush failed"); // madsim requires flush

    info!(response = "Hello World", "Server sent response");
}

#[tracing::instrument]
pub async fn client(server_addr: SocketAddr) {
    let mut stream = TcpStream::connect(server_addr).await.expect("Error connecting");

    stream.write_all(b"I'm ready").await.expect("client: write failed");
    stream.flush().await.expect("client: flush failed"); // madsim requires flush
    debug!(msg = "I'm ready", "Sent to server");

    let mut buffer = [0u8; 64];
    let bytes_read = stream.read(&mut buffer).await.expect("client: read failed");
    let msg = String::from_utf8_lossy(&buffer[..bytes_read]);

    info!(?msg, "Response received");
}

#[cfg(test)]
mod tests {
    #[cfg(madsim)]
    #[madsim::test]
    async fn simple_send() {
        use std::{net::SocketAddr, sync::Arc};

        use tokio::sync::Barrier;
        use tracing_subscriber::fmt;

        use super::*;

        let _ = fmt()
            .with_max_level(tracing::Level::DEBUG)
            .try_init();

        let server_addr: SocketAddr = "10.0.0.1:8800".parse().expect("Socket Address Parse failed");

        let handle = madsim::runtime::Handle::current();

        let server_node = handle
            .create_node()
            .name("server")
            .ip("10.0.0.1".parse().expect("Socket Address Parse failed"))
            .build();
        let client_node = handle
            .create_node()
            .name("client")
            .ip("10.0.0.2".parse().expect("Socket Address Parse failed"))
            .build();

        let barrier_server = Arc::new(Barrier::new(2));
        let barrier_client = barrier_server.clone();

        let server_task = server_node.spawn(async move {
            // Wait until the listener is bound before allowing the client to
            // connect.
            let listener = TcpListener::bind(server_addr).await.expect("Bind listener failed");
            barrier_server.wait().await;

            serve(listener).await;
        });
        let client_task = client_node.spawn(async move {
            barrier_client.wait().await;
            client(server_addr).await;
        });

        let (server_result, client_result) = tokio::join!(server_task, client_task);

        // Check tasks for panics/errors
        server_result.expect("Error from server task");
        client_result.expect("Error from client task");
    }
}

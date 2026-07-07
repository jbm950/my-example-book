use std::net::SocketAddr;

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};
use tracing::{error, info, trace};

const CHANNEL_CAPACITY: usize = 100;
const READ_BUF_SIZE: usize = 128;

#[derive(Clone, Debug)]
struct Message {
    sender: SocketAddr,
    bytes: Vec<u8>,
}

async fn handle_client(
    socket: TcpStream,
    addr: SocketAddr,
    broadcast_tx: broadcast::Sender<Message>,
) {
    let (mut reader, mut writer) = socket.into_split();
    let mut buf = [0u8; READ_BUF_SIZE];

    let mut broadcast_rx = broadcast_tx.subscribe();

    loop {
        tokio::select! {
            result = reader.read(&mut buf) => {
                match result {
                    Ok(0) => {
                        info!("Client Disconnected: {addr}");
                        break;
                    }
                    Ok(n) => {
                        let bytes = buf[..n].to_vec();
                        trace!("[{addr}] {n} bytes");
                        let _ = broadcast_tx.send(Message { sender: addr, bytes });
                    }

                    Err(e) => {
                        error!("Error reading from {addr}: {e}");
                        break;
                    }
                }
            }

            result = broadcast_rx.recv() => {
                match result {
                    Ok(msg) => {
                        if msg.sender != addr {
                            if writer.write_all(&msg.bytes).await.is_err() {
                                break;
                            }
                        }
                    }

                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        error!("Client {addr} lagged by {n} messages");
                    }

                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }
}

pub async fn tcp_bus(listener: TcpListener) -> io::Result<()> {
    // broadcast::channel requires an initial receiver; real ones come from `.subscribe()` per
    // client.
    let (broadcast_tx, _rx) = broadcast::channel::<Message>(CHANNEL_CAPACITY);

    loop {
        let (socket, addr) = listener.accept().await?;
        info!("New client connected: {addr}");

        tokio::spawn(handle_client(socket, addr, broadcast_tx.clone()));
    }
}

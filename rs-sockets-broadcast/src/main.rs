use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

const CHANNEL_CAPACITY: usize = 100;

#[derive(Clone, Debug)]
struct Message {
    sender: SocketAddr,
    text: String,
}

async fn handle_client(
    socket: TcpStream,
    addr: SocketAddr,
    broadcast_tx: broadcast::Sender<Message>,
) {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader).lines();

    let mut broadcast_rx = broadcast_tx.subscribe();

    loop {
        tokio::select! {
            result = reader.next_line() => {
                match result {
                    Ok(Some(line)) => {
                        println!("[{addr}] {line}");
                        let _ = broadcast_tx.send(Message {sender: addr, text: line});
                    }

                    Ok(None) => {
                        println!("Client disconnected: {addr}");
                        break;
                    }

                    Err(e) => {
                        eprintln!("Error reading from {addr}: {e}");
                        break;
                    }
                }
            }

            result = broadcast_rx.recv() => {
                match result {
                    Ok(msg) => {
                        if msg.sender != addr {
                            let out = format!("[{}] {}\n", msg.sender, msg.text);
                            if writer.write_all(out.as_bytes()).await.is_err() {
                                break;
                            }
                        }
                    }

                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        eprintln!("Client {addr} lagged by {n} messages");
                    }

                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    // broadcast::channel requires an initial receiver; real ones come from `.subscribe()` per
    // client.
    let (broadcast_tx, _rx) = broadcast::channel::<Message>(CHANNEL_CAPACITY);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New client connected: {addr}");

        tokio::spawn(handle_client(socket, addr, broadcast_tx.clone()));
    }
}

use tokio::{sync::broadcast, task::JoinSet};

async fn worker(id: usize, mut rx: broadcast::Receiver<&'static str>) {
    loop {
        match rx.recv().await {
            Ok(msg) => {
                println!("Worker {id} received message {msg}");

                if msg == "Stop" {
                    break;
                }
            }

            Err(broadcast::error::RecvError::Lagged(n)) => {
                println!("Worker {id} lagged {n} messages")
            }

            Err(broadcast::error::RecvError::Closed) => break,
        }
    }
}

#[tokio::main]
async fn main() {
    // Initial rx must be held or the channel will be closed. Additional receivers are created
    // using tx.subscribe().
    let (tx, _rx) = broadcast::channel::<&'static str>(16);

    let mut workers = JoinSet::new();
    for i in 0..3 {
        let rx = tx.subscribe();
        workers.spawn(worker(i, rx));
    }

    for msg in [
        "Hello, world!",
        "Another message",
        "Everyone hearing me?",
        "Stop",
    ] {
        tx.send(msg).expect("Error sending message");
    }

    workers.join_all().await;
}

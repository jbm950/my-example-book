use std::time::Duration;

use tokio::{
    sync::{mpsc, oneshot},
    time::{interval, sleep},
};

async fn send_shutdown(tx: oneshot::Sender<()>) {
    sleep(Duration::from_secs(5)).await;
    tx.send(()).expect("Failed to send shutdown signal");
}

async fn send_data(tx: mpsc::Sender<u32>) {
    let events = [
        // (data, sleep_time in milliseconds)
        (5_u32, 700),
        (32, 150),
        (43, 500),
        (52, 2300),
        (13, 500),
        (23, 300),
    ];

    for (value, sleep_time) in events {
        sleep(Duration::from_millis(sleep_time)).await;
        tx.send(value).await.expect("Failed to send data");
    }
}

#[tokio::main]
async fn main() {
    let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
    let mut heartbeat = interval(Duration::from_secs(1));
    let capacity = 32;
    let (data_tx, mut data_rx) = mpsc::channel(capacity);

    tokio::spawn(send_shutdown(shutdown_tx));
    tokio::spawn(send_data(data_tx));

    loop {
        tokio::select! {
            _ = &mut shutdown_rx => {  // Oneshot has to be polled as a mutable reference
                println!("Shutdown detected");
                break;
            },

            Some(val) = data_rx.recv() => println!("Received data: {val}"),

            _ = heartbeat.tick() => println!("Heartbeat"),
        }
    }
}

use std::time::Duration;

use tokio::{signal, time::interval};
use tokio_util::{sync::CancellationToken, task::TaskTracker};

async fn repeating_task(id: usize, cancel_token: CancellationToken) {
    let mut ticker = interval(Duration::from_secs(1));
    ticker.tick().await;  // Consume the immediate tick
    
    loop {
        tokio::select! {
            _ = ticker.tick() => {
                println!("Interval {id} ticked");
            }

            _ = cancel_token.cancelled() => {
                println!("Shutdown detected in task {id}. Wrapping up resources");
                tokio::time::sleep(Duration::from_secs(1)).await;
                println!("Cleanup complete for task {id}!");
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let tracker = TaskTracker::new();

    let cancel_token = CancellationToken::new();

    for i in 0..3 {
        tracker.spawn(repeating_task(i, cancel_token.clone()));
    }

    match signal::ctrl_c().await {
        Ok(()) => println!("Ctrl C detected in main task! Shutting down."),
        Err(err) => eprintln!("Unable to listen for shutdown signal: {}", err),
    }
    cancel_token.cancel();

    tracker.close();  // prevent any new tasks from being added
    tracker.wait().await;

    println!("Closing program. All tasks wrapped up");
}

use std::sync::Arc;

use tokio::{
    sync::Notify,
    task::JoinSet,
    time::{Duration, sleep},
};

#[tokio::main]
async fn main() {
    let notify = Arc::new(Notify::new());

    let worker_notify = notify.clone();

    let worker = tokio::spawn(async move {
        println!("Worker waiting");
        worker_notify.notified().await;
        println!("Worker woke up!");
    });

    sleep(Duration::from_secs(1)).await;

    println!("Sending notification");
    notify.notify_one();
    worker.await.unwrap();

    let mut workers = JoinSet::new();
    for i in 0..10 {
        let task_notify = notify.clone();
        workers.spawn(async move {
            println!("Task {i} waiting");
            task_notify.notified().await;
            println!("Task {i} woke up!");
        });
    }

    sleep(Duration::from_secs(1)).await;

    println!("Waking first task (first task registered)");
    notify.notify_one();

    sleep(Duration::from_secs(1)).await;

    println!("Waking last task (last task registered)");
    notify.notify_last();

    sleep(Duration::from_secs(1)).await;

    println!("Waking remaining tasks");
    notify.notify_waiters();
    workers.join_all().await;

    notify.notify_one();
    notify.notified().await;
    println!(
        "One notify is queued. Waiter does not need to already be waiting"
    );
}

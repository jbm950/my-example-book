use tokio::time::Duration;
use tracing::{Level, debug, info, trace};

// Gives the span a name separate from the function name
#[tracing::instrument(name="Custom Name")]
async fn task_1() {
    for i in 0..10 {
        trace!(loop = i, "Task 1");
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[tracing::instrument]
async fn task_2(worker_num: u32) {
    for i in 0..5 {
        debug!(iteration = i, "Task 2");
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

#[tracing::instrument]
async fn task_3() {
    let mut interval = tokio::time::interval(Duration::from_millis(1500));
    for i in 0..7 {
        interval.tick().await;
        child_task(i).await;
    }
}

#[tracing::instrument(skip(i), fields(in_val = %i))]
async fn child_task(i: u32) {
    info!("Task 3 did something!");
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let task1 = tokio::spawn(task_1());
    let task2_1 = tokio::spawn(task_2(1));
    let task2_2 = tokio::spawn(task_2(2));
    let task3 = tokio::spawn(task_3());

    let _ = tokio::join!(task1, task2_1, task2_2, task3);
}

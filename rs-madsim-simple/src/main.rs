use tokio::time::Duration;
use tracing::{Level, info};

async fn task_1() {
    for i in 0..5 {
        info!("Task 1: {:}", i);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let task1 = tokio::spawn(task_1());

    let _ = task1.await;
}

#[cfg(test)]
mod test {
    use super::task_1;

    #[tokio::test]
    async fn test_task_1() {
        let task1 = tokio::spawn(task_1());

        let _ = task1.await;
    }
}

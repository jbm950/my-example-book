use std::time::Duration;

async fn task_1() {
    for i in 0..10 {
        println!("Task 1: {:}", i);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}


async fn task_2() {
    for i in 0..5 {
        println!("Task 2: {:}", i);
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}


#[tokio::main]
async fn main() {
    let task1 = tokio::spawn(task_1());
    let task2 = tokio::spawn(task_2());

    let _ = task1.await;
    let _ = task2.await;
}

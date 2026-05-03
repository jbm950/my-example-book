use std::time::Duration;


async fn task_1() -> String {
    for i in 0..10 {
        println!("Task 1: {}", i);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    "Done with Task 1".into()
}


async fn task_2() -> String {
    for i in 0..5 {
        println!("Task 2: {}", i);
        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    "Done with Task 2".into()
}


async fn task_3() -> String {
    tokio::time::sleep(Duration::from_secs(1)).await;
    "Done with Task 3".into()
}


#[tokio::main]
async fn main() {
    let task1 = tokio::spawn(task_1());

    let val2 = task_2().await;
    let val1 = task1.await;

    let results3 = tokio::join!(task_3(), task_3(), task_3());

    println!("\n\n");

    println!("{}", val1.unwrap());
    println!("{}", val2);
    println!("{:?}", results3);
}

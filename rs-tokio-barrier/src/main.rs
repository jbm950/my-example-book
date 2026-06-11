use std::{sync::Arc, time::Duration};

use tokio::{sync::Barrier, task::JoinSet, time::sleep};

const NUM_TASKS: u64 = 3;

#[tokio::main]
async fn main() {
    let barrier = Arc::new(Barrier::new(NUM_TASKS as usize));

    let mut tasks = JoinSet::new();
    for id in 0..NUM_TASKS {
        let barrier = Arc::clone(&barrier);

        tasks.spawn(async move {
            println!("Task {id}: starting");

            sleep(Duration::from_secs(id)).await;

            println!("Task {id}: waiting at first barrier");

            let phase_1_result = barrier.wait().await;
            if phase_1_result.is_leader() {
                println!("Task {id} is the leader");
            }

            println!("Task {id}: phase 1 complete");

            // Reverse arrival order for phase 2
            sleep(Duration::from_secs(2 - id)).await;

            println!("Task {id}: waiting at second barrier");

            // Barriers are reusable
            let phase_2_result = barrier.wait().await;
            if phase_2_result.is_leader() {
                println!("Task {id} is the leader");
            }

            println!("Task {id}: phase 2 complete");
        });
    }

    while let Some(response) = tasks.join_next().await {
        response.expect("Task panicked");
    }
}

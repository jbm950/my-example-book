use std::time::Duration;

use tokio::{
    sync::mpsc,
    time::{MissedTickBehavior, interval},
};
use tracing::error;

use crate::protocol::CommandMessage;

pub struct PeriodicCommand {
    pub interval: Duration,
    pub command: CommandMessage,
}

pub async fn run(schedule: PeriodicCommand, tx: mpsc::Sender<CommandMessage>) {
    let mut timer = interval(schedule.interval);

    // Don't attempt to catch up after delays.
    // Resume periodic execution from the current time.
    timer.set_missed_tick_behavior(MissedTickBehavior::Delay);

    timer.tick().await; // Consume initial tick

    loop {
        timer.tick().await;

        if tx.send(schedule.command.clone()).await.is_err() {
            error!("Bus controller channel closed. Scheduler exiting");
            break;
        };
    }
}

use std::time::Duration;

use tokio::{
    sync::{mpsc, oneshot},
    time::{Interval, MissedTickBehavior, interval},
};
use tracing::{debug, error};

use crate::{
    app::Transaction,
    devices::gps::GpsTelemetry,
    devices::power::PowerTelemetry,
    protocol::StatusMessage,
    protocol::{CmdWord, CommandMessage, Subaddress, TxRx},
};

pub async fn run(tx: mpsc::Sender<Transaction>) {
    let mut rt5_7t_interval = make_interval(Duration::from_secs(1));
    rt5_7t_interval.tick().await; // Consume initial tick

    let mut rt13_13t_interval = make_interval(Duration::from_secs(3));
    rt13_13t_interval.tick().await; // Consume initial tick

    loop {
        tokio::select! {
            _ = rt5_7t_interval.tick() => {
                let cmd_msg = CommandMessage {
                    word: CmdWord::new(5, Subaddress { address: 7, tr: TxRx::T }, 3),
                    data: Vec::new(),
                };

                let Some(status_msg) = request(&tx, cmd_msg).await else {
                    break;
                };

                match PowerTelemetry::from_data_words(&status_msg.data) {
                    Ok(t) => debug!(telemetry = ?t, "RT 5 Power Telemetry"),
                    Err(e) => error!(error = ?e, "RT5 decode failed")
                };

            }

            _ = rt13_13t_interval.tick() => {
                let cmd_msg = CommandMessage {
                    word: CmdWord::new(13, Subaddress { address: 13, tr: TxRx::T }, 15),
                    data: Vec::new(),
                };

                let Some(status_msg) = request(&tx, cmd_msg).await else {
                    break;
                };

                let gps_telemetry = GpsTelemetry::from_data_words(status_msg.data);
                debug!(telemetry = ?gps_telemetry, "RT13 GPS Telemetry");
            }
        }
    }
}

fn make_interval(duration: Duration) -> Interval {
    let mut out = interval(duration);

    // Don't attempt to catch up after delays.
    // Resume periodic execution from the current time.
    out.set_missed_tick_behavior(MissedTickBehavior::Delay);

    out
}

async fn request(tx: &mpsc::Sender<Transaction>, cmd_msg: CommandMessage) -> Option<StatusMessage> {
    let (resp_tx, resp_rx) = oneshot::channel::<StatusMessage>();
    if tx
        .send(Transaction {
            command: cmd_msg,
            response_tx: resp_tx,
        })
        .await
        .is_err()
    {
        error!("Bus controller channel closed. Scheduler exiting");
        return None;
    };

    match resp_rx.await {
        Ok(status) => Some(status),
        Err(_) => {
            error!("Bus controller dropped response channel.");
            None
        }
    }
}

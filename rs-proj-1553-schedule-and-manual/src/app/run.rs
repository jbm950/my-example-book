use std::{net::SocketAddr, time::Duration};

use tokio::sync::mpsc;
use tracing::{debug, error};

use crate::{
    devices::{gps::GpsTelemetry, power::{Fault, PowerCommand, PowerTelemetry}},
    net::TcpBusController,
    protocol::{CmdWord, CommandMessage, Subaddress, Transaction, TxRx},
    runtime::scheduler::{self, PeriodicCommand},
};

const POWER_RT: u8 = 5;
const GPS_RT: u8 = 13;

pub async fn run(server_addr: SocketAddr) {
    let periodic_cmds = [
        PeriodicCommand {
            interval: Duration::from_secs(1),
            command: CommandMessage {
                word: CmdWord::new(
                    5,
                    Subaddress {
                        address: 7,
                        tr: TxRx::T,
                    },
                    3,
                ),
                data: Vec::new(),
            },
        },
        PeriodicCommand {
            interval: Duration::from_secs(2),
            command: CommandMessage {
                word: CmdWord::new(
                    13,
                    Subaddress {
                        address: 13,
                        tr: TxRx::T,
                    },
                    15,
                ),
                data: Vec::new(),
            },
        },
    ];

    let (command_tx, command_rx) = mpsc::channel::<CommandMessage>(32);

    for periodic_cmd in periodic_cmds {
        tokio::spawn(scheduler::run(periodic_cmd, command_tx.clone()));
    }

    let (transactions_tx, transactions_rx) = mpsc::channel::<Transaction>(32);
    let bus_controller = TcpBusController::new(server_addr).await.unwrap();
    let controller_task = tokio::spawn(bus_controller.run(command_rx, transactions_tx));

    tokio::spawn(handle_transaction(transactions_rx));

    tokio::time::sleep(Duration::from_secs(7)).await;

    command_tx.send(CommandMessage {
            word: CmdWord::new(5, Subaddress { address: 5, tr: TxRx::R }, 1),
            data: PowerCommand::InjectFault(Fault::OverTemp).to_data_words(),
    }).await.expect("Bus controller task exited unexpectedly");

    tokio::time::sleep(Duration::from_secs(7)).await;

    command_tx.send(CommandMessage {
            word: CmdWord::new(5, Subaddress { address: 5, tr: TxRx::R }, 1),
            data: PowerCommand::ClearFault.to_data_words(),
    }).await.expect("Bus controller task exited unexpectedly");

    if let Ok(Err(e)) = controller_task.await {
        error!(error = %e, "Bus controller exited with error");
    }
}

async fn handle_transaction(mut transaction_rx: mpsc::Receiver<Transaction>) {
    while let Some(transaction) = transaction_rx.recv().await {
        match transaction.command.word.rt_addr {
            POWER_RT => {
                // The 5R command doesn't need to be handled
                if matches!(
                    transaction.command.word.subaddr,
                    Subaddress {
                        address: 7,
                        tr: TxRx::T,
                    }
                ) {
                    match PowerTelemetry::from_data_words(&transaction.status.data) {
                        Ok(t) => debug!(telemetry = ?t, "RT 5 Power Telemetry"),
                        Err(e) => error!(error = ?e, "RT5 decode failed"),
                    };
                }
            }
            GPS_RT => { // Only 1 subaddress currently implemented for GPS, 13T
                let gps_telemetry = GpsTelemetry::from_data_words(&transaction.status.data);
                debug!(telemetry = ?gps_telemetry, "RT13 GPS Telemetry");
            }
            unknown_addr => {
                error!(unknown_addr, "Unknown RT address in transaction")
            }
        }
    }
}

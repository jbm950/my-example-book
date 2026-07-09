use std::net::SocketAddr;

use tokio::sync::mpsc;

use crate::{
    app::{config, state::App, tui},
    net::TcpBusController,
    protocol::{CommandMessage, Transaction},
    runtime::scheduler,
};

pub async fn run(server_addr: SocketAddr) {
    let (command_tx, command_rx) = mpsc::channel::<CommandMessage>(32);

    for periodic_cmd in config::periodic_commands() {
        tokio::spawn(scheduler::run(periodic_cmd, command_tx.clone()));
    }

    let (transactions_tx, mut transactions_rx) = mpsc::channel::<Transaction>(32);
    let bus_controller = TcpBusController::new(server_addr).await.unwrap();
    let _ = tokio::spawn(bus_controller.run(command_rx, transactions_tx));

    let mut app_state = App::new(command_tx);
    let _ = tui::run_app(&mut app_state, &mut transactions_rx).await;
}

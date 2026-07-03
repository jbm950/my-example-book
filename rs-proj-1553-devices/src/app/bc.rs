use std::{net::SocketAddr, time::Duration};

use tokio::{
    io,
    time::{MissedTickBehavior, interval},
};
use tracing::trace;

use crate::{
    devices::gps::GpsTelemetry,
    devices::power::{Fault, PowerCommand, PowerTelemetry},
    net::TcpBusController,
    protocol::StatusMessage,
    protocol::{CmdWord, CommandMessage, DataWord, Subaddress, TxRx},
};

pub async fn bus_controller(server_addr: SocketAddr) -> io::Result<()> {
    let mut tcp_bc = TcpBusController::new(server_addr).await?;

    let mut rt5_5r_interval = interval(Duration::from_secs(7));
    rt5_5r_interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    rt5_5r_interval.tick().await; // Consume initial tick

    let mut rt5_7t_interval = interval(Duration::from_secs(1));
    rt5_7t_interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    rt5_7t_interval.tick().await; // Consume initial tick

    let mut rt13_13t_interval = interval(Duration::from_secs(3));
    rt13_13t_interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    rt13_13t_interval.tick().await; // Consume initial tick

    loop {
        tokio::select! {
            _ = rt5_5r_interval.tick() => {
                let cmd_msg = CommandMessage {
                    word: CmdWord::new(5, Subaddress { address: 5, tr: TxRx::R }, 1),
                    data: PowerCommand::InjectFault(Fault::OverTemp).to_data_words(),
                };

                do_transaction(&mut tcp_bc, cmd_msg).await?;
            }

            _ = rt5_7t_interval.tick() => {
                let cmd = CmdWord::new(5, Subaddress { address: 7, tr: TxRx::T }, 3);
                let data: Vec<DataWord> = Vec::new();
                let cmd_msg = CommandMessage {
                    word: cmd,
                    data
                };

                let status_msg = do_transaction(&mut tcp_bc, cmd_msg).await?;
                let power_telemetry = PowerTelemetry::from_data_words(&status_msg.data);
                println!("{:?}", power_telemetry.unwrap());
            }

            _ = rt13_13t_interval.tick() => {
                let cmd = CmdWord::new(13, Subaddress { address: 13, tr: TxRx::T }, 15);
                let data: Vec<DataWord> = Vec::new();
                let cmd_msg = CommandMessage {
                    word: cmd,
                    data
                };

                let status_msg = do_transaction(&mut tcp_bc, cmd_msg).await?;
                let gps_telemetry = GpsTelemetry::from_data_words(status_msg.data);
                println!("{:?}", gps_telemetry);
            }
        }
    }
}

async fn do_transaction(
    bc: &mut TcpBusController,
    cmd_msg: CommandMessage,
) -> io::Result<StatusMessage> {
    trace!(
        "Sending RT {} Subaddr {} Tx/RX {:?} command",
        cmd_msg.word.rt_addr, cmd_msg.word.subaddr.address, cmd_msg.word.subaddr.tr
    );
    let status_msg = bc.transaction(cmd_msg).await?;
    trace!("Received response. Data words {:?}", status_msg.data);

    Ok(status_msg)
}

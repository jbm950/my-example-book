use std::{net::SocketAddr, time::Duration};

use tokio::{
    io,
    time::{MissedTickBehavior, interval},
};
use tracing::trace;

use crate::protocol::{CmdWord, CommandMessage, DataWord, Subaddress, TxRx};
use crate::{devices::gps::GpsTelemetry, net::TcpBusController, protocol::StatusMessage};

const RT5_INTERVAL: f64 = 3.0; // Hertz
const RT13_INTERVAL: f64 = 1.0; // Hertz

pub async fn bus_controller(server_addr: SocketAddr) -> io::Result<()> {
    let mut tcp_bc = TcpBusController::new(server_addr).await?;

    let mut rt5_5r_interval = interval(Duration::from_millis((1000.0 / RT5_INTERVAL) as u64));
    rt5_5r_interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    let mut rt5_7t_interval = interval(Duration::from_millis((300.0 / RT5_INTERVAL) as u64));
    rt5_7t_interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    let mut rt13_13t_interval = interval(Duration::from_millis((2000.0 / RT13_INTERVAL) as u64));
    rt13_13t_interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        tokio::select! {
            _ = rt5_5r_interval.tick() => {
                let cmd_msg = CommandMessage {
                    word: CmdWord::new(5, Subaddress { address: 5, tr: TxRx::R }, 3),
                    data: [5_u16, 5, 4].map(DataWord::from).to_vec()
                };

                let _ = do_transaction(&mut tcp_bc, cmd_msg).await?;
            }

            _ = rt5_7t_interval.tick() => {
                let cmd = CmdWord::new(5, Subaddress { address: 7, tr: TxRx::T }, 5);
                let data: Vec<DataWord> = Vec::new();
                let cmd_msg = CommandMessage {
                    word: cmd,
                    data
                };

                let _ = do_transaction(&mut tcp_bc, cmd_msg).await?;
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

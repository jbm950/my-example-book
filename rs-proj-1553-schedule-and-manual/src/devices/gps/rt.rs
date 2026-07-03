use std::net::SocketAddr;

use tokio::{io, time::Instant};
use tracing::{error, info};

use crate::{
    devices::gps::{GpsTelemetry, GpsTime, Position, Velocity},
    net::TcpRt,
    protocol::{StatusMessage, StatusWord, Subaddress, TxRx},
};

pub async fn run(server: SocketAddr, rt_id: u8) -> io::Result<()> {
    let mut last_update = Instant::now();

    let mut gps_telemetry = GpsTelemetry {
        time: GpsTime {
            week: 5,
            seconds_of_week: 12345.67,
        },
        position: Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        velocity: Velocity {
            x: 0.5,
            y: 0.0,
            z: 0.7,
        },
    };

    let mut rt = TcpRt::new(server, rt_id).await?;
    let status_word = StatusWord {
        rt_addr: rt_id,
        msg_error: false,
        service_req: false,
        broadcast_received: false,
        busy_bit: false,
        subsystem_flag: false,
        dyn_bus_accept: false,
        terminal_flag: false,
    };

    loop {
        let cmd_msg = rt.read().await?;

        let data = match cmd_msg.word.subaddr {
            Subaddress {
                address: 13,
                tr: TxRx::T,
            } => {
                gps_telemetry.update(last_update.elapsed());
                last_update = Instant::now();

                let data = gps_telemetry.to_data_words();
                info!(
                    "Received 13T command. Sending telemetry data: {:?}",
                    gps_telemetry
                );
                data
            }

            subaddr => {
                error!(
                    "Unimplemented subaddress {} TxRx {:?}",
                    subaddr.address, subaddr.tr
                );
                error!("Unhandled command {:?}", cmd_msg.word);
                continue;
            }
        };

        rt.write(StatusMessage {
            word: status_word,
            data,
        })
        .await?;
    }
}

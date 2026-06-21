use std::{net::SocketAddr, time::Duration};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::{MissedTickBehavior, interval, timeout},
};
use tracing::{error, trace, warn};

use crate::protocol::{CmdWord, StatusWord, Subaddress, TxRx};

const READ_BUF_SIZE: usize = 128;
const TIMEOUT_DURATION: Duration = Duration::from_millis(50);

const RT5_INTERVAL: f64 = 3.0; // Hertz
const RT13_INTERVAL: f64 = 1.0; // Hertz

pub async fn bus_controller(server: SocketAddr) {
    let socket = TcpStream::connect(server)
        .await
        .expect("Failed to connect to bus");
    let (mut reader, mut writer) = socket.into_split();
    let mut buf = vec![0u8; READ_BUF_SIZE];

    let mut rt5_interval = interval(Duration::from_millis((1000.0 / RT5_INTERVAL) as u64));
    rt5_interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let mut rt13_interval = interval(Duration::from_millis((1000.0 / RT13_INTERVAL) as u64));
    rt13_interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        let rt_num = tokio::select! {
            _ = rt5_interval.tick() => {
                let cmd: u16 = CmdWord::new(5, Subaddress { address: 5, tr: TxRx::R }, 0).into();
                match writer.write_all(&cmd.to_be_bytes()).await {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Failed sending command to RT 5: {e}");
                        continue;
                    }
                };
                trace!("Sent 5R CMD to RT 5");
                5

            }

            _ = rt13_interval.tick() => {
                let cmd: u16 = CmdWord::new(13, Subaddress { address: 13, tr: TxRx::R }, 0).into();
                match writer.write_all(&cmd.to_be_bytes()).await {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Failed sending command to RT 13: {e}");
                        continue;
                    }
                };
                trace!("Sent 13R CMD to RT 13");
                13
            }
        };

        // Would want to build up responses with a loop in case they're sent independently in a
        // fuller example. Leaving simple for this example.
        match timeout(TIMEOUT_DURATION, reader.read(&mut buf)).await {
            Ok(Ok(0)) => {
                error!("RT {rt_num} connection is closed");
            }

            Ok(Ok(bytes_read)) => {
                if bytes_read < 2 {
                    warn!("Not enough bytes read for a StatusWord");
                } else {
                    let status_word = StatusWord::try_from(&buf[..2]).unwrap();
                    if status_word.rt_addr != rt_num {
                        warn!("Status word does not correspond to last Command sent!");
                    } else {
                        trace!("Received status word for {rt_num}");
                    }
                }
            }

            Ok(Err(e)) => {
                error!("Error while reading RT response: {e}");
            }

            Err(_) => {
                error!("Timeout error waiting for response from RT {rt_num}");
            }
        }
    }
}

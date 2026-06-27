use std::net::SocketAddr;

use tokio::io;
use tracing::{error, info};

use crate::{
    net::TcpRt,
    protocol::{DataWord, StatusMessage, StatusWord, Subaddress, TxRx},
};

pub async fn rt(server: SocketAddr, rt_id: u8) -> io::Result<()> {
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

        match cmd_msg.word.subaddr {
            Subaddress {
                address: 5,
                tr: TxRx::R,
            } => {
                info!("Received 5R command with data: {:?}", cmd_msg.data);
                rt.write(StatusMessage {
                    word: status_word,
                    data: Vec::new(),
                })
                .await?;
            }

            Subaddress {
                address: 7,
                tr: TxRx::T,
            } => {
                let data = vec![DataWord::from(7_u16); cmd_msg.word.word_count as usize];

                info!("Received 7T command. Sending data: {:?}", data);
                rt.write(StatusMessage {
                    word: status_word,
                    data,
                })
                .await?;
            }

            Subaddress {
                address: 13,
                tr: TxRx::T,
            } => {
                let data = vec![DataWord::from(13_u16); cmd_msg.word.word_count as usize];

                info!("Received 13T command. Sending data: {:?}", data);
                rt.write(StatusMessage {
                    word: status_word,
                    data,
                })
                .await?;
            }

            subaddr => {
                error!(
                    "Unimplemented subaddress {} TxRx {:?}",
                    subaddr.address, subaddr.tr
                );
                error!("Unhandled command {:?}", cmd_msg.word);
            }
        }
    }
}

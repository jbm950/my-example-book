use std::net::SocketAddr;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tracing::{error, info, trace, warn};

use crate::protocol::{CmdWord, StatusWord};

const READ_BUF_SIZE: usize = 128;

// Example just acknowledges commands addressed to it rather than responding to the specific
// subaddress.
pub async fn rt(server: SocketAddr, rt_addr: u8) {
    let socket = TcpStream::connect(server)
        .await
        .expect("Failed to connect to bus");
    let (mut reader, mut writer) = socket.into_split();
    let mut buf = vec![0u8; READ_BUF_SIZE];

    loop {
        // Would want to build up responses with a loop in case they're sent independently in a
        // fuller example. Leaving simple for this example.
        match reader.read(&mut buf).await {
            Ok(0) => {
                error!("Bus connection closed");
                break;
            }

            Ok(bytes_read) => {
                if bytes_read < 2 {
                    warn!("Not enough bytes read for a CmdWord");
                    continue;
                } else {
                    let cmd_word = CmdWord::try_from(&buf[..2]).unwrap();

                    if cmd_word.rt_addr != rt_addr {
                        trace!(
                            "Detected command word not for this RT: This RT {rt_addr}, Cmd for {}",
                            cmd_word.rt_addr
                        );
                        continue;
                    } else {
                        info!(
                            "Detected commandword for this RT: Subaddress {}{:?}",
                            cmd_word.subaddr.address, cmd_word.subaddr.tr
                        );
                    }
                }
            }

            Err(e) => {
                error!("Error while reading RT response: {e}");
                continue;
            }
        }

        let status: u16 = StatusWord {
            rt_addr,
            msg_error: false,
            service_req: false,
            broadcast_received: false,
            busy_bit: false,
            subsystem_flag: false,
            dyn_bus_accept: false,
            terminal_flag: false,
        }
        .into();
        match writer.write_all(&status.to_be_bytes()).await {
            Ok(_) => {}
            Err(e) => {
                error!("Failed responding with status: {e}");
                continue;
            }
        };
    }
}

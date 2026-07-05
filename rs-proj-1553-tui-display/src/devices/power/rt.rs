use std::net::SocketAddr;

use tokio::{
    io,
    time::{Duration, interval},
};
use tracing::error;

use crate::{
    devices::power::{Fault, Power, PowerCommand, PowerMode},
    net::TcpRt,
    protocol::{CommandMessage, StatusMessage, StatusWord, TxRx},
};

const UPDATE_RATE: Duration = Duration::from_millis(100);

pub async fn run(server: SocketAddr, rt_id: u8) -> io::Result<()> {
    let mut power_state = Power {
        mode: PowerMode::Discharging,
        charge_percent: 97.0,
        temperature_c: Power::AMBIENT_TEMP,
        fault: Fault::None,
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

    let mut update_interval = interval(UPDATE_RATE);
    update_interval.tick().await; // First tick consumes immediately

    loop {
        tokio::select! {
            _ = update_interval.tick() => {
                power_state.update(UPDATE_RATE);
            }

            // Example doesn't implement error handling here for simplicity
            Ok(cmd_msg) = rt.read() => {
                let response = process_command_message(&mut power_state, status_word, cmd_msg);
                rt.write(response).await?;
            }
        }
    }
}

fn process_command_message(
    power_state: &mut Power,
    status_word: StatusWord,
    cmd_msg: CommandMessage,
) -> StatusMessage {
    match (cmd_msg.word.subaddr.address, cmd_msg.word.subaddr.tr) {
        (5, TxRx::R) => {
            match PowerCommand::from_data_words(&cmd_msg.data) {
                Ok(cmd) => power_state.handle_command(cmd),
                Err(err) => {
                    error!("Failed to parse power command: {err:?}");
                }
            };

            // In a more complete 1553 implementation the message error bit could be set here.
            StatusMessage::empty(status_word)
        }

        (7, TxRx::T) => StatusMessage {
            word: status_word,
            data: power_state.telemetry().to_data_words(),
        },

        (address, tr) => {
            error!("Unimplemented subaddress {} TxRx {:?}", address, tr);
            error!("Unhandled command {:?}", cmd_msg.word);
            StatusMessage::empty(status_word)
        }
    }
}

use std::time::Duration;

use crate::{
    protocol::{CmdWord, CommandMessage, Subaddress, TxRx},
    runtime::scheduler::PeriodicCommand,
};

pub fn periodic_commands() -> Vec<PeriodicCommand> {
    vec![
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
    ]
}

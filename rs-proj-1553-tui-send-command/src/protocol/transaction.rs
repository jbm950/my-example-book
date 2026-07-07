use crate::protocol::{CommandMessage, StatusMessage};

pub struct Transaction {
    pub command: CommandMessage,
    pub status: StatusMessage,
}

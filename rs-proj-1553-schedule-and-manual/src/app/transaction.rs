use tokio::sync::oneshot;

use crate::protocol::{CommandMessage, StatusMessage};

pub struct Transaction {
    pub command: CommandMessage,
    pub response_tx: oneshot::Sender<StatusMessage>,
}

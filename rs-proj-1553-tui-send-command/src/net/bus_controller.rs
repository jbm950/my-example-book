use std::net::SocketAddr;

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc,
};
use tracing::{debug, trace};

use crate::protocol::{
    CommandMessage, DataWord, StatusMessage, StatusWord, Transaction, TxRx, WORD_SIZE,
};

/// Bus Controller for a 1553 over Ethernet implementation.
pub struct TcpBusController {
    bus: TcpStream,
}

impl TcpBusController {
    pub async fn new(server_addr: SocketAddr) -> Result<Self, io::Error> {
        let bus = TcpStream::connect(server_addr).await?;
        Ok(Self { bus })
    }

    #[tracing::instrument(
        level = "trace",
        skip(self, cmd_msg),
        fields(
            rt = cmd_msg.word.rt_addr,
            subaddr = cmd_msg.word.subaddr.address,
            tr = ?cmd_msg.word.subaddr.tr
        )
    )]
    async fn transaction(&mut self, cmd_msg: CommandMessage) -> io::Result<Transaction> {
        trace!("Starting transaction");
        self.send_cmd(&cmd_msg).await?;

        let status_word = self.read_status_word().await?;
        let data_words = match cmd_msg.word.subaddr.tr {
            // RT is commanded to transmit data words
            TxRx::T => self.read_data_words(cmd_msg.word.word_count).await?,
            // RT received data words that were included in the CommandMessage
            TxRx::R => Vec::new(),
        };

        trace!("Transaction complete");
        Ok(Transaction {
            command: cmd_msg,
            status: StatusMessage {
                word: status_word,
                data: data_words,
            },
        })
    }

    pub async fn run(
        mut self,
        mut command_rx: mpsc::Receiver<CommandMessage>,
        transactions_tx: mpsc::Sender<Transaction>,
    ) -> io::Result<()> {
        while let Some(command) = command_rx.recv().await {
            let transaction = self.transaction(command).await?;
            if transactions_tx.send(transaction).await.is_err() {
                debug!("Transaction receiver dropped. Bus controller exiting.");
                break;
            }
        }

        Ok(())
    }

    async fn send_cmd(&mut self, cmd_msg: &CommandMessage) -> io::Result<()> {
        let msg_bytes = cmd_msg.encode();
        trace!(
            "Writing {} bytes (1 command word and {} data words)",
            msg_bytes.len(),
            cmd_msg.data.len(),
        );
        self.bus.write_all(&msg_bytes).await
    }

    async fn read_status_word(&mut self) -> io::Result<StatusWord> {
        let mut buf = [0_u8; WORD_SIZE];
        self.bus.read_exact(&mut buf).await?;
        Ok(StatusWord::from(buf))
    }

    async fn read_data_words(&mut self, count: u8) -> io::Result<Vec<DataWord>> {
        trace!("Reading {count} data words");

        let mut data_buf = vec![0_u8; count as usize * WORD_SIZE];
        self.bus.read_exact(&mut data_buf).await?;

        Ok(data_buf
            .chunks_exact(WORD_SIZE)
            .map(|bytes| DataWord::from_be_bytes([bytes[0], bytes[1]]))
            .collect())
    }
}

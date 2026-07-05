use std::net::SocketAddr;

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tracing::{debug, trace};

use crate::protocol::{CmdWord, CommandMessage, DataWord, StatusMessage, TxRx, WORD_SIZE};

/// Remote terminal for a 1553 over Ethernet implementation.
pub struct TcpRt {
    rt_id: u8,
    bus: TcpStream,
}

impl TcpRt {
    pub async fn new(server_addr: SocketAddr, rt_id: u8) -> Result<Self, io::Error> {
        let bus = TcpStream::connect(server_addr).await?;
        Ok(Self { rt_id, bus })
    }

    pub async fn read(&mut self) -> io::Result<CommandMessage> {
        loop {
            let cmd_word = self.read_cmd_word().await?;

            if cmd_word.rt_addr != self.rt_id {
                debug!(
                    "Detected command word not for this RT: This RT {}, Cmd for {}",
                    self.rt_id, cmd_word.rt_addr,
                );
                self.discard_transaction(cmd_word.word_count).await?;
                continue;
            }

            let data_words = match cmd_word.subaddr.tr {
                TxRx::R => self.read_data_words(cmd_word.word_count).await?,
                TxRx::T => Vec::new(),
            };
            return Ok(CommandMessage {
                word: cmd_word,
                data: data_words,
            });
        }
    }

    async fn read_cmd_word(&mut self) -> io::Result<CmdWord> {
        let mut buf = [0_u8; WORD_SIZE];
        self.bus.read_exact(&mut buf).await?;
        Ok(CmdWord::from(buf))
    }

    async fn read_data_words(&mut self, count: u8) -> io::Result<Vec<DataWord>> {
        trace!("Reading {count} data words for RT {}", self.rt_id);

        let mut data_buf = vec![0_u8; count as usize * WORD_SIZE];
        self.bus.read_exact(&mut data_buf).await?;

        Ok(data_buf
            .chunks_exact(WORD_SIZE)
            .map(|bytes| DataWord::from_be_bytes([bytes[0], bytes[1]]))
            .collect())
    }

    async fn discard_transaction(&mut self, num_data_words: u8) -> io::Result<()> {
        // 1 status word + num_data_words
        let mut discard = vec![0; WORD_SIZE * (1 + num_data_words as usize)];
        trace!(
            "Discarding {} bytes (1 status word and {} data words)",
            discard.len(),
            num_data_words
        );
        self.bus.read_exact(&mut discard).await?;
        Ok(())
    }

    pub async fn write(&mut self, status_msg: StatusMessage) -> io::Result<()> {
        let msg_bytes = status_msg.encode();
        trace!(
            "Writing {} bytes (1 status word and {} data words)",
            msg_bytes.len(),
            status_msg.data.len(),
        );
        self.bus.write_all(&msg_bytes).await?;
        Ok(())
    }
}

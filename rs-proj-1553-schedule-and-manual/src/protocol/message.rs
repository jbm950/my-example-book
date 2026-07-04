use crate::protocol::{CmdWord, DataWord, ProtocolWord, StatusWord};

#[derive(Clone)]
pub struct Message<W> {
    pub word: W,
    pub data: Vec<DataWord>,
}

pub type CommandMessage = Message<CmdWord>;
pub type StatusMessage = Message<StatusWord>;

impl<W: ProtocolWord> Message<W> {
    pub fn empty(word: W) -> Self {
        Self {
            word,
            data: Vec::new(),
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(2 + self.data.len() * 2);

        out.extend(self.word.to_be_bytes());

        for word in &self.data {
            out.extend(word.to_be_bytes());
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_works_correctly() {
        // Did not check that the "number of data words" field in the command
        // message actually corresponds to the number in the data words in the
        // assert block. Just checking the byte arrays.
        assert_eq!(
            CommandMessage {
                word: CmdWord::from([105_u8, 73]),
                data: vec![
                    DataWord(u16::from_be_bytes([0_u8, 15])),
                    DataWord(u16::from_be_bytes([27_u8, 81])),
                    DataWord(u16::from_be_bytes([212_u8, 124])),
                ]
            }
            .encode(),
            [105_u8, 73, 0, 15, 27, 81, 212, 124]
        );

        assert_eq!(
            StatusMessage {
                word: StatusWord::from([92u8, 12]),
                data: vec![
                    DataWord(u16::from_be_bytes([98_u8, 243])),
                    DataWord(u16::from_be_bytes([167_u8, 192])),
                    DataWord(u16::from_be_bytes([51_u8, 0])),
                    DataWord(u16::from_be_bytes([86_u8, 69])),
                    DataWord(u16::from_be_bytes([15_u8, 162])),
                ]
            }
            .encode(),
            [92_u8, 12, 98, 243, 167, 192, 51, 0, 86, 69, 15, 162]
        );
    }
}

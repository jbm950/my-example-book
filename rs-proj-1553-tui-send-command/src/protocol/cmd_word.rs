use crate::protocol::ProtocolWord;

use super::{
    WORD_SIZE,
    support::{extract_field, extract_flag},
};

// Values are the actual protocol bit values for each type.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum TxRx {
    T = 1,
    R = 0,
}

impl From<bool> for TxRx {
    fn from(value: bool) -> Self {
        if value { Self::T } else { Self::R }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Subaddress {
    pub address: u8,
    pub tr: TxRx,
}

const RT_ADDR_POSITION: u8 = 11;
const TX_RX_POSITION: u8 = 10;
const SUBADDRESS_POSITION: u8 = 5;
const WORD_COUNT_POSITION: u8 = 0;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CmdWord {
    pub rt_addr: u8,
    pub subaddr: Subaddress,
    pub word_count: u8,
}

impl CmdWord {
    pub fn new(rt_addr: u8, subaddr: Subaddress, word_count: u8) -> Self {
        Self {
            rt_addr,
            subaddr,
            word_count,
        }
    }
}

impl ProtocolWord for CmdWord {
    fn to_be_bytes(self) -> [u8; WORD_SIZE] {
        u16::from(self).to_be_bytes()
    }
}

impl From<CmdWord> for u16 {
    fn from(cmd_word: CmdWord) -> Self {
        // 1553 protocl states 32 words -> value of 0
        let word_count = if cmd_word.word_count == 32 {
            0
        } else {
            cmd_word.word_count
        };
        ((cmd_word.rt_addr as u16) << RT_ADDR_POSITION)
            | ((cmd_word.subaddr.tr as u16) << TX_RX_POSITION)
            | ((cmd_word.subaddr.address as u16) << SUBADDRESS_POSITION)
            | (word_count as u16)
    }
}

impl From<u16> for CmdWord {
    fn from(cmd_word: u16) -> Self {
        let rt_addr = extract_field(cmd_word, RT_ADDR_POSITION, 0b1_1111) as u8;
        let tr = TxRx::from(extract_flag(cmd_word, TX_RX_POSITION));
        let subaddr = extract_field(cmd_word, SUBADDRESS_POSITION, 0b1_1111) as u8;

        let mut word_count = extract_field(cmd_word, WORD_COUNT_POSITION, 0b1_1111) as u8;
        if word_count == 0 {
            word_count = 32
        }; // 1553 protocol states value of 0 -> 32 words

        Self::new(
            rt_addr,
            Subaddress {
                address: subaddr,
                tr,
            },
            word_count,
        )
    }
}

impl From<[u8; WORD_SIZE]> for CmdWord {
    fn from(bytes: [u8; WORD_SIZE]) -> CmdWord {
        Self::from(u16::from_be_bytes(bytes))
    }
}

impl TryFrom<&[u8]> for CmdWord {
    type Error = std::array::TryFromSliceError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let arr: [u8; WORD_SIZE] = bytes.try_into()?;
        Ok(Self::from(arr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_cmd_word_correctly() {
        let cmd_word: u16 = CmdWord::new(
            1,
            Subaddress {
                address: 22,
                tr: TxRx::R,
            },
            13,
        )
        .into();

        assert_eq!(cmd_word, 2765);
    }

    #[test]
    fn to_be_bytes_works_correctly() {
        assert_eq!(
            [10u8, 205],
            CmdWord::new(
                1,
                Subaddress {
                    address: 22,
                    tr: TxRx::R
                },
                13
            )
            .to_be_bytes()
        );
    }

    #[test]
    fn build_cmd_word_from_bytes_correctly() {
        assert_eq!(
            CmdWord::from([10u8, 205]),
            CmdWord::new(
                1,
                Subaddress {
                    address: 22,
                    tr: TxRx::R
                },
                13
            )
        );
    }

    #[test]
    fn build_cmd_word_from_byte_slice_correctly() {
        let bytes = [10u8, 205, 12, 100, 24];
        assert_eq!(
            CmdWord::try_from(&bytes[0..2]).unwrap(),
            CmdWord::new(
                1,
                Subaddress {
                    address: 22,
                    tr: TxRx::R
                },
                13
            )
        );
    }
}

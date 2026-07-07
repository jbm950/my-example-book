use crate::protocol::ProtocolWord;

use super::{
    WORD_SIZE,
    support::{extract_field, extract_flag},
};

const RT_ADDR_POSITION: u8 = 11;
const MSG_ERROR_POSITION: u8 = 10;
const SERVICE_REQ_POSITION: u8 = 8;
const BROADCAST_RECV_POSITION: u8 = 4;
const BUSY_BIT_POSITION: u8 = 3;
const SUBSYSTEM_FLAG_POSITION: u8 = 2;
const DYN_BUS_ACCEPT_POSITION: u8 = 1;
const TERMINAL_FLAG_POSITION: u8 = 0;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StatusWord {
    pub rt_addr: u8,
    pub msg_error: bool,
    pub service_req: bool,
    pub broadcast_received: bool,
    pub busy_bit: bool,
    pub subsystem_flag: bool,
    pub dyn_bus_accept: bool,
    pub terminal_flag: bool,
}

impl ProtocolWord for StatusWord {
    fn to_be_bytes(self) -> [u8; WORD_SIZE] {
        u16::from(self).to_be_bytes()
    }
}

impl From<StatusWord> for u16 {
    fn from(status_word: StatusWord) -> Self {
        // Instrumentation bit at position 10 shall be 0
        ((status_word.rt_addr as u16) << RT_ADDR_POSITION)
            | ((status_word.msg_error as u16) << MSG_ERROR_POSITION)
            | ((status_word.service_req as u16) << SERVICE_REQ_POSITION)
            | ((status_word.broadcast_received as u16) << BROADCAST_RECV_POSITION)
            | ((status_word.busy_bit as u16) << BUSY_BIT_POSITION)
            | ((status_word.subsystem_flag as u16) << SUBSYSTEM_FLAG_POSITION)
            | ((status_word.dyn_bus_accept as u16) << DYN_BUS_ACCEPT_POSITION)
            | (status_word.terminal_flag as u16)
    }
}

impl From<u16> for StatusWord {
    fn from(status_word: u16) -> Self {
        let rt_addr = extract_field(status_word, RT_ADDR_POSITION, 0b1_1111) as u8;
        let msg_error = extract_flag(status_word, MSG_ERROR_POSITION);
        let service_req = extract_flag(status_word, SERVICE_REQ_POSITION);
        let broadcast_received = extract_flag(status_word, BROADCAST_RECV_POSITION);
        let busy_bit = extract_flag(status_word, BUSY_BIT_POSITION);
        let subsystem_flag = extract_flag(status_word, SUBSYSTEM_FLAG_POSITION);
        let dyn_bus_accept = extract_flag(status_word, DYN_BUS_ACCEPT_POSITION);
        let terminal_flag = extract_flag(status_word, TERMINAL_FLAG_POSITION);

        StatusWord {
            rt_addr,
            msg_error,
            service_req,
            broadcast_received,
            busy_bit,
            subsystem_flag,
            dyn_bus_accept,
            terminal_flag,
        }
    }
}

impl From<[u8; WORD_SIZE]> for StatusWord {
    fn from(bytes: [u8; WORD_SIZE]) -> StatusWord {
        Self::from(u16::from_be_bytes(bytes))
    }
}

impl TryFrom<&[u8]> for StatusWord {
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
    fn pack_status_word_correctly() {
        let status_word: u16 = StatusWord {
            rt_addr: 5,
            msg_error: false,
            service_req: true,
            broadcast_received: false,
            busy_bit: true,
            subsystem_flag: true,
            dyn_bus_accept: false,
            terminal_flag: false,
        }
        .into();
        assert_eq!(status_word, 10508)
    }

    #[test]
    fn to_be_bytes_works_correctly() {
        assert_eq!(
            [40u8, 0],
            StatusWord {
                rt_addr: 5,
                msg_error: false,
                service_req: false,
                broadcast_received: false,
                busy_bit: false,
                subsystem_flag: false,
                dyn_bus_accept: false,
                terminal_flag: false
            }
            .to_be_bytes()
        );
    }
    #[test]
    fn build_status_word_from_bytes_correctly() {
        assert_eq!(
            StatusWord::from([40u8, 0]),
            StatusWord {
                rt_addr: 5,
                msg_error: false,
                service_req: false,
                broadcast_received: false,
                busy_bit: false,
                subsystem_flag: false,
                dyn_bus_accept: false,
                terminal_flag: false
            }
        );
    }

    #[test]
    fn build_status_word_from_byte_slice_correctly() {
        let bytes = [40u8, 0, 13, 52, 4];
        assert_eq!(
            StatusWord::try_from(&bytes[0..2]).unwrap(),
            StatusWord {
                rt_addr: 5,
                msg_error: false,
                service_req: false,
                broadcast_received: false,
                busy_bit: false,
                subsystem_flag: false,
                dyn_bus_accept: false,
                terminal_flag: false
            }
        );
    }
}

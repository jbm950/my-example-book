use super::WORD_SIZE;

pub trait ProtocolWord: Copy {
    fn to_be_bytes(self) -> [u8; WORD_SIZE];
}

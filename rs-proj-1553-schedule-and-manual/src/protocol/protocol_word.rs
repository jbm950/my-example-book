pub trait ProtocolWord: Copy {
    fn to_be_bytes(self) -> [u8; 2];
}

pub(super) fn extract_flag(word: u16, bit_position: u8) -> bool {
    ((word >> bit_position) & 1) != 0
}

pub(super) fn extract_field(word: u16, bit_position: u8, mask: u16) -> u16 {
    (word >> bit_position) & mask
}

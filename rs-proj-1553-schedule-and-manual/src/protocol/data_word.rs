use super::WORD_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataWord(pub u16);

impl DataWord {
    pub fn to_be_bytes(self) -> [u8; WORD_SIZE] {
        self.0.to_be_bytes()
    }

    pub fn from_be_bytes(bytes: [u8; WORD_SIZE]) -> Self {
        Self(u16::from_be_bytes(bytes))
    }
}

impl From<u16> for DataWord {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_be_bytes_works_correctly() {
        assert_eq!(DataWord(2765_u16).to_be_bytes(), [10_u8, 205])
    }
}

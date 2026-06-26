#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataWord(pub u16);

impl DataWord {
    pub fn to_be_bytes(self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_be_bytes_works_correctly() {
        assert_eq!(
            DataWord(2765_u16).to_be_bytes(),
            [10_u8, 205]
        )
    }
}


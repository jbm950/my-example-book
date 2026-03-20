#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn serialize(self) -> [u8; 14] {
        let mut result = [0u8; 14];
        result[0..2].copy_from_slice(b"hs");
        result[2..6].copy_from_slice(&self.x.to_le_bytes());
        result[6..10].copy_from_slice(&self.y.to_le_bytes());
        result[10..].copy_from_slice(&self.z.to_le_bytes());
        result
    }

    pub fn deserialize(packet_bytes: [u8; 14]) -> Position {
        Position {
            x: f32::from_le_bytes(packet_bytes[2..6].try_into().unwrap()),
            y: f32::from_le_bytes(packet_bytes[6..10].try_into().unwrap()),
            z: f32::from_le_bytes(packet_bytes[10..14].try_into().unwrap()),
        }
    }
}

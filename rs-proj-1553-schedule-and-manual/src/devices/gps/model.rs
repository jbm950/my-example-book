use tokio::time::Duration;

use crate::protocol::DataWord;

#[derive(Debug)]
pub struct GpsTelemetry {
    pub time: GpsTime,
    pub position: Position,
    pub velocity: Velocity,
}

impl GpsTelemetry {
    pub fn update(&mut self, elapsed: Duration) {
        self.time.seconds_of_week += elapsed.as_secs_f32();
        self.position.x += self.velocity.x * elapsed.as_secs_f32();
        self.position.y += self.velocity.y * elapsed.as_secs_f32();
        self.position.z += self.velocity.z * elapsed.as_secs_f32();
    }

    fn to_be_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(
            GpsTime::GPS_TIME_SIZE + Position::POSITION_SIZE + Velocity::VELOCITY_SIZE,
        );

        out.extend(self.time.to_be_bytes());
        out.extend(self.position.to_be_bytes());
        out.extend(self.velocity.to_be_bytes());

        out
    }

    pub fn to_data_words(&self) -> Vec<DataWord> {
        self.to_be_bytes()
            .chunks_exact(2)
            .map(|chunk| {
                let bytes: [u8; 2] = chunk.try_into().unwrap();
                DataWord::from(u16::from_be_bytes(bytes))
            })
            .collect()
    }

    fn from_be_bytes(bytes: &[u8]) -> Self {
        let t = GpsTime::GPS_TIME_SIZE;
        let p = Position::POSITION_SIZE;

        Self {
            time: GpsTime::from_be_bytes(&bytes[..t]),
            position: Position::from_be_bytes(&bytes[t..t + p]),
            velocity: Velocity::from_be_bytes(&bytes[t + p..]),
        }
    }

    pub fn from_data_words(words: &[DataWord]) -> Self {
        let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_be_bytes()).collect();

        Self::from_be_bytes(&bytes)
    }
}

#[derive(Debug)]
pub struct GpsTime {
    pub week: u16,
    pub seconds_of_week: f32,
}

impl GpsTime {
    const GPS_TIME_SIZE: usize = 6; // u16 + f32

    fn to_be_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(GpsTime::GPS_TIME_SIZE);

        out.extend(self.week.to_be_bytes());
        out.extend(self.seconds_of_week.to_be_bytes());

        out
    }

    fn from_be_bytes(bytes: &[u8]) -> Self {
        Self {
            week: u16::from_be_bytes(bytes[0..2].try_into().unwrap()),
            seconds_of_week: f32::from_be_bytes(bytes[2..6].try_into().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    const POSITION_SIZE: usize = 12; // f32 x 3

    fn to_be_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(Position::POSITION_SIZE);

        out.extend(self.x.to_be_bytes());
        out.extend(self.y.to_be_bytes());
        out.extend(self.z.to_be_bytes());

        out
    }

    fn from_be_bytes(bytes: &[u8]) -> Self {
        Self {
            x: f32::from_be_bytes(bytes[0..4].try_into().unwrap()),
            y: f32::from_be_bytes(bytes[4..8].try_into().unwrap()),
            z: f32::from_be_bytes(bytes[8..12].try_into().unwrap()),
        }
    }
}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Velocity {
    const VELOCITY_SIZE: usize = 12; // f32 x 3

    fn to_be_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(Velocity::VELOCITY_SIZE);

        out.extend(self.x.to_be_bytes());
        out.extend(self.y.to_be_bytes());
        out.extend(self.z.to_be_bytes());

        out
    }

    fn from_be_bytes(bytes: &[u8]) -> Self {
        Self {
            x: f32::from_be_bytes(bytes[0..4].try_into().unwrap()),
            y: f32::from_be_bytes(bytes[4..8].try_into().unwrap()),
            z: f32::from_be_bytes(bytes[8..12].try_into().unwrap()),
        }
    }
}

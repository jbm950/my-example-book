use tokio::time::Duration;

use crate::protocol::DataWord;

pub enum PowerTelemetryError {
    UnknownMode(u8),
    UnknownFault(u8),
}

#[derive(Clone, Copy)]
pub enum PowerMode {
    Idle,
    Charging,
    Discharging,
}

impl PowerMode {
    fn encode(self) -> u8 {
        match self {
            Self::Idle => 0,
            Self::Charging => 1,
            Self::Discharging => 2,
        }
    }

    fn decode(byte: u8) -> Result<Self, PowerTelemetryError> {
        Ok(match byte {
            0 => Self::Idle,
            1 => Self::Charging,
            2 => Self::Discharging,
            _ => {
                return Err(PowerTelemetryError::UnknownMode(byte));
            }
        })
    }
}

#[derive(Clone, Copy)]
pub enum Fault {
    None,
    OverTemp,
    UnderVoltage,
}

impl Fault {
    fn encode(self) -> u8 {
        match self {
            Self::None => 0,

            Self::OverTemp => 1,

            Self::UnderVoltage => 2,
        }
    }

    fn decode(byte: u8) -> Result<Self, PowerTelemetryError> {
        Ok(match byte {
            0 => Self::None,
            1 => Self::OverTemp,
            2 => Self::UnderVoltage,
            _ => {
                return Err(PowerTelemetryError::UnknownFault(byte));
            }
        })
    }
}

struct Power {
    mode: PowerMode,
    charge_percent: f32,
    temperature_c: f32,
    fault: Fault,
}

impl Power {
    const MIN_CHARGE: f32 = 10.0;
    const CHARGE_RATE: f32 = 100.0 / 120.0; // Fully charge in 2 minutes
    const DISCHARGE_RATE: f32 = 100.0 / 60.0; // Fully discharge in 60 seconds

    const MAX_TEMP: f32 = 35.0;
    const AMBIENT_TEMP: f32 = 22.0;
    const CHARGE_TEMP_RATE: f32 = 0.1;
    const DISCHARGE_TEMP_RATE: f32 = 0.05;
    const IDLE_TEMP_RATE: f32 = -0.01;

    pub fn update(&mut self, elapsed: Duration) {
        let elapsed_sec = elapsed.as_secs_f32();

        match self.mode {
            PowerMode::Idle => {
                self.temperature_c = (self.temperature_c + elapsed_sec * Self::IDLE_TEMP_RATE)
                    .max(Self::AMBIENT_TEMP);
            }
            PowerMode::Charging => {
                self.charge_percent =
                    (self.charge_percent + elapsed_sec * Self::CHARGE_RATE).min(100.0);
                self.temperature_c += elapsed_sec * Self::CHARGE_TEMP_RATE;
            }
            PowerMode::Discharging => {
                self.charge_percent =
                    (self.charge_percent - elapsed_sec * Self::DISCHARGE_RATE).max(0.0);
                self.temperature_c += elapsed_sec * Self::DISCHARGE_TEMP_RATE;
            }
        }

        if self.temperature_c > Self::MAX_TEMP {
            self.fault = Fault::OverTemp;
        }

        if self.charge_percent < Self::MIN_CHARGE && !matches!(self.mode, PowerMode::Charging) {
            self.fault = Fault::UnderVoltage;
        }

        if !matches!(self.fault, Fault::None) {
            self.mode = PowerMode::Idle;
        }
    }

    pub fn handle_command(&mut self, command: PowerCommand) {
        match command {
            PowerCommand::SetMode(mode) => {
                if matches!(self.fault, Fault::None) {
                    self.mode = mode;
                }
            }

            PowerCommand::ClearFault => {
                self.fault = Fault::None;
            }

            PowerCommand::InjectFault(fault) => {
                self.fault = fault;
                self.mode = PowerMode::Idle;
            }
        }
    }

    pub fn telemetry(&self) -> PowerTelemetry {
        PowerTelemetry {
            mode: self.mode,
            charge_percent: self.charge_percent as u8,
            temperature_c: self.temperature_c,
            fault: self.fault,
        }
    }
}

struct PowerTelemetry {
    mode: PowerMode,
    charge_percent: u8, // Smaller type than simulated state to conserve bandwidth
    temperature_c: f32,
    fault: Fault,
}

impl PowerTelemetry {
    const SIZE: usize = 6;

    pub fn to_be_bytes(&self) -> [u8; Self::SIZE] {
        let first_byte = (self.mode.encode() << 4) | self.fault.encode();
        let temp_bytes = self.temperature_c.to_be_bytes();

        [
            first_byte,
            self.charge_percent,
            temp_bytes[0],
            temp_bytes[1],
            temp_bytes[2],
            temp_bytes[3],
        ]

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

    pub fn from_be_bytes(bytes: &[u8]) -> Result<Self, PowerTelemetryError> {
        Ok(Self {
            mode: PowerMode::decode((bytes[0] >> 4) & 0b1111)?,
            charge_percent: bytes[1],
            temperature_c: f32::from_be_bytes(bytes[2..6].try_into().unwrap()),
            fault: Fault::decode(bytes[0] & 0b1111)?,
        })
    }

    pub fn from_data_words(words: &[DataWord]) -> Result<Self, PowerTelemetryError> {
        let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_be_bytes()).collect();

        Self::from_be_bytes(&bytes)
    }
}

enum PowerCommand {
    SetMode(PowerMode),
    ClearFault,
    InjectFault(Fault),
}

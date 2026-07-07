use std::fmt::{self, Display, Formatter};

use tokio::time::Duration;

use crate::protocol::DataWord;

#[derive(Debug)]
pub enum PowerParseError {
    InvalidLength { expected: usize, actual: usize },
    UnknownMode(u8),
    UnknownFault(u8),
    UnknownCommand(u8),
}

#[derive(Debug, Clone, Copy)]
pub enum PowerMode {
    Idle,
    Charging,
    Discharging,
}

impl PowerMode {
    const IDLE: u8 = 0;
    const CHARGING: u8 = 1;
    const DISCHARGING: u8 = 2;

    fn encode(self) -> u8 {
        match self {
            Self::Idle => Self::IDLE,
            Self::Charging => Self::CHARGING,
            Self::Discharging => Self::DISCHARGING,
        }
    }

    fn decode(byte: u8) -> Result<Self, PowerParseError> {
        Ok(match byte {
            Self::IDLE => Self::Idle,
            Self::CHARGING => Self::Charging,
            Self::DISCHARGING => Self::Discharging,
            _ => {
                return Err(PowerParseError::UnknownMode(byte));
            }
        })
    }
}

impl Display for PowerMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Idle => write!(f, "Idle"),
            Self::Charging => write!(f, "Charging"),
            Self::Discharging => write!(f, "Discharging"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Fault {
    None,
    OverTemp,
    UnderVoltage,
}

impl Fault {
    const NONE: u8 = 0;
    const OVERTEMP: u8 = 1;
    const UNDERVOLTAGE: u8 = 2;

    fn encode(self) -> u8 {
        match self {
            Self::None => Self::NONE,
            Self::OverTemp => Self::OVERTEMP,
            Self::UnderVoltage => Self::UNDERVOLTAGE,
        }
    }

    fn decode(byte: u8) -> Result<Self, PowerParseError> {
        Ok(match byte {
            Self::NONE => Self::None,
            Self::OVERTEMP => Self::OverTemp,
            Self::UNDERVOLTAGE => Self::UnderVoltage,
            _ => {
                return Err(PowerParseError::UnknownFault(byte));
            }
        })
    }
}

impl Display for Fault {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::OverTemp => write!(f, "OverTemp"),
            Self::UnderVoltage => write!(f, "UnderVoltage"),
        }
    }
}

pub struct Power {
    pub mode: PowerMode,
    pub charge_percent: f32,
    pub temperature_c: f32,
    pub fault: Fault,
}

impl Power {
    const MIN_CHARGE: f32 = 10.0;
    const CHARGE_RATE: f32 = 100.0 / 120.0; // Fully charge in 2 minutes
    const DISCHARGE_RATE: f32 = 100.0 / 60.0; // Fully discharge in 60 seconds

    const MAX_TEMP: f32 = 35.0;
    pub const AMBIENT_TEMP: f32 = 22.0;
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

#[derive(Debug)]
pub struct PowerTelemetry {
    pub mode: PowerMode,
    pub charge_percent: u8, // Smaller type than simulated state to conserve bandwidth
    pub temperature_c: f32,
    pub fault: Fault,
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

    pub fn from_be_bytes(bytes: &[u8]) -> Result<Self, PowerParseError> {
        if bytes.len() != Self::SIZE {
            return Err(PowerParseError::InvalidLength {
                expected: Self::SIZE,
                actual: bytes.len(),
            });
        }

        Ok(Self {
            mode: PowerMode::decode((bytes[0] >> 4) & 0b1111)?,
            charge_percent: bytes[1],
            temperature_c: f32::from_be_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]),
            fault: Fault::decode(bytes[0] & 0b1111)?,
        })
    }

    pub fn from_data_words(words: &[DataWord]) -> Result<Self, PowerParseError> {
        let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_be_bytes()).collect();

        Self::from_be_bytes(&bytes)
    }
}

#[derive(Clone, Copy)]
pub enum PowerCommand {
    SetMode(PowerMode),
    ClearFault,
    InjectFault(Fault),
}

impl PowerCommand {
    const SIZE: usize = 2;

    const SET_MODE: u8 = 0;
    const CLEAR_FAULT: u8 = 1;
    const INJECT_FAULT: u8 = 2;

    pub fn to_be_bytes(&self) -> [u8; Self::SIZE] {
        match self {
            Self::SetMode(mode) => [Self::SET_MODE, mode.encode()],
            Self::ClearFault => [Self::CLEAR_FAULT, 0],
            Self::InjectFault(fault) => [Self::INJECT_FAULT, fault.encode()],
        }
    }

    pub fn to_data_words(&self) -> Vec<DataWord> {
        let [a, b] = self.to_be_bytes();
        vec![DataWord::from(u16::from_be_bytes([a, b]))]
    }

    pub fn from_be_bytes(bytes: &[u8]) -> Result<Self, PowerParseError> {
        if bytes.len() != Self::SIZE {
            return Err(PowerParseError::InvalidLength {
                expected: Self::SIZE,
                actual: bytes.len(),
            });
        }

        Ok(match bytes[0] {
            Self::SET_MODE => Self::SetMode(PowerMode::decode(bytes[1])?),
            Self::CLEAR_FAULT => Self::ClearFault,
            Self::INJECT_FAULT => Self::InjectFault(Fault::decode(bytes[1])?),
            _ => {
                return Err(PowerParseError::UnknownCommand(bytes[0]));
            }
        })
    }

    pub fn from_data_words(words: &[DataWord]) -> Result<Self, PowerParseError> {
        let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_be_bytes()).collect();

        Self::from_be_bytes(&bytes)
    }
}

impl Display for PowerCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::SetMode(mode) => {
                write!(f, "Set Mode: {mode}")
            }
            Self::ClearFault => {
                write!(f, "Clear Fault")
            }
            Self::InjectFault(fault) => {
                write!(f, "Inject Fault: {fault}")
            }
        }
    }
}

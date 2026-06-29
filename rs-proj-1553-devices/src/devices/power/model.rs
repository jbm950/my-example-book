use tokio::time::Duration;

#[derive(Clone, Copy)]
pub enum PowerMode {
    Idle,
    Charging,
    Discharging,
}

#[derive(Clone, Copy)]
pub enum Fault {
    None,
    OverTemp,
    UnderVoltage,
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
                self.temperature_c = (self.temperature_c + elapsed_sec * Power::IDLE_TEMP_RATE)
                    .max(Power::AMBIENT_TEMP);
            }
            PowerMode::Charging => {
                self.charge_percent =
                    (self.charge_percent + elapsed_sec * Power::CHARGE_RATE).min(100.0);
                self.temperature_c += elapsed_sec * Power::CHARGE_TEMP_RATE;
            }
            PowerMode::Discharging => {
                self.charge_percent =
                    (self.charge_percent - elapsed_sec * Power::DISCHARGE_RATE).max(0.0);
                self.temperature_c += elapsed_sec * Power::DISCHARGE_TEMP_RATE;
            }
        }

        if self.temperature_c > Power::MAX_TEMP {
            self.fault = Fault::OverTemp;
        }

        if self.charge_percent < Power::MIN_CHARGE && !matches!(self.mode, PowerMode::Charging) {
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

enum PowerCommand {
    SetMode(PowerMode),
    ClearFault,
    InjectFault(Fault),
}

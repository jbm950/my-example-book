use ratatui::crossterm::event::KeyEvent;
use tracing::{debug, error};

use crate::{
    app::tui::CommandPanel, devices::{gps::GpsTelemetry, power::PowerTelemetry}, protocol::{Subaddress, Transaction, TxRx}
};

const POWER_RT: u8 = 5;
const GPS_RT: u8 = 13;

#[derive(Default)]
pub struct App {
    pub power_telemetry: Option<PowerTelemetry>,
    pub power_commands: CommandPanel,
    pub gps_telemetry: Option<GpsTelemetry>,
    pub exit: bool,
}

impl App {
    pub fn handle_key(&mut self, _key_event: KeyEvent) {
        self.exit = true;
    }

    pub fn handle_transaction(&mut self, transaction: Transaction) {
        match transaction.command.word.rt_addr {
            POWER_RT => {
                // The 5R command doesn't need to be handled
                if matches!(
                    transaction.command.word.subaddr,
                    Subaddress {
                        address: 7,
                        tr: TxRx::T,
                    }
                ) {
                    match PowerTelemetry::from_data_words(&transaction.status.data) {
                        Ok(power_telemetry) => {
                            debug!(telemetry = ?power_telemetry, "RT 5 Power Telemetry");
                            self.power_telemetry = Some(power_telemetry);
                        }
                        Err(e) => error!(error = ?e, "RT5 decode failed"),
                    };
                }
            }
            GPS_RT => {
                // Only 1 subaddress currently implemented for GPS, 13T
                let gps_telemetry = GpsTelemetry::from_data_words(&transaction.status.data);
                debug!(telemetry = ?gps_telemetry, "RT13 GPS Telemetry");
                self.gps_telemetry = Some(gps_telemetry);
            }
            unknown_addr => {
                error!(unknown_addr, "Unknown RT address in transaction")
            }
        }
    }
}

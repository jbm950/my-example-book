use ratatui::crossterm::event::{KeyCode, KeyEvent};
use tokio::sync::mpsc;
use tracing::{debug, error};

use crate::{
    app::tui::CommandPanel,
    devices::{
        gps::GpsTelemetry,
        power::{PowerCommand, PowerTelemetry},
    },
    protocol::{CmdWord, CommandMessage, Subaddress, Transaction, TxRx},
};

const POWER_RT: u8 = 5;
const GPS_RT: u8 = 13;

pub struct App {
    pub power_telemetry: Option<PowerTelemetry>,
    pub power_commands: CommandPanel,
    pub gps_telemetry: Option<GpsTelemetry>,
    commands_tx: mpsc::Sender<CommandMessage>,
    pub exit: bool,
}

impl App {
    pub fn new(commands_tx: mpsc::Sender<CommandMessage>) -> Self {
        Self {
            power_telemetry: None,
            power_commands: CommandPanel::default(),
            gps_telemetry: None,
            commands_tx,
            exit: false,
        }
    }

    pub async fn handle_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('j') => self.power_commands.next(),
            KeyCode::Char('k') => self.power_commands.previous(),
            KeyCode::Enter => {
                if let Some(command) = self.power_commands.selected() {
                    if let Err(err) = self
                        .commands_tx
                        .send(Self::power_command_message(command))
                        .await
                    {
                        error!(?err, "Failed to send command");
                        self.exit = true;
                    };
                }
            }
            _ => self.exit = true,
        }
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

    fn power_command_message(power_command: PowerCommand) -> CommandMessage {
        CommandMessage {
            word: CmdWord::new(
                POWER_RT,
                Subaddress {
                    address: 5,
                    tr: TxRx::R,
                },
                1,
            ),
            data: power_command.to_data_words(),
        }
    }
}

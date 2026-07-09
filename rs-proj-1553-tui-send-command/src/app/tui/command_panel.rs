use ratatui::widgets::{ListItem, ListState};

use crate::devices::power::{Fault, PowerCommand, PowerMode};

pub struct CommandPanel {
    commands: [PowerCommand; 6],
    state: ListState,
}

impl Default for CommandPanel {
    fn default() -> Self {
        Self {
            commands: [
                PowerCommand::SetMode(PowerMode::Idle),
                PowerCommand::SetMode(PowerMode::Charging),
                PowerCommand::SetMode(PowerMode::Discharging),
                PowerCommand::ClearFault,
                PowerCommand::InjectFault(Fault::OverTemp),
                PowerCommand::InjectFault(Fault::UnderVoltage),
            ],
            state: ListState::default().with_selected(Some(0)),
        }
    }
}

impl CommandPanel {
    pub fn next(&mut self) {
        self.state.select_next();
    }

    pub fn previous(&mut self) {
        self.state.select_previous();
    }

    pub fn selected(&self) -> Option<PowerCommand> {
        self.state.selected().map(|index| self.commands[index])
    }

    pub fn list_commands(&self) -> impl Iterator<Item = ListItem<'static>> {
        self.commands
            .iter()
            .map(|cmd| ListItem::new(cmd.to_string()))
    }

    pub fn state(&mut self) -> &mut ListState {
        &mut self.state
    }
}

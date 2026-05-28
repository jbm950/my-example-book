use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyCode, KeyModifiers},
    widgets::Paragraph,
};

fn run_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        let mut message: Vec<String> = vec![];
        if let Some(key) = event::read()?.as_key_press_event() {
            message.push(key.code.to_string());

            if key.modifiers.contains(KeyModifiers::CONTROL) {
                message.push("CTRL".into());
            }

            if key.modifiers.contains(KeyModifiers::ALT) {
                message.push("Alt".into());
            }

            if key.modifiers.contains(KeyModifiers::SHIFT) {
                message.push("Shift".into());
            }

            if matches!(key.code, KeyCode::Esc) {
                break;
            }
        }

        terminal.draw(|frame| ui(frame, message))?;
    }
    Ok(())
}

fn ui(frame: &mut Frame, message: Vec<String>) {
    frame.render_widget(Paragraph::new(message.join(", ")), frame.area());
}

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| run_app(terminal))?;
    Ok(())
}

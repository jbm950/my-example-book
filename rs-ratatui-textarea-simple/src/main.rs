use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyCode},
    widgets::{Block, Borders},
};
use ratatui_textarea::TextArea;

struct App {
    input_field: TextArea<'static>,
}

impl App {
    fn new() -> Self {
        let mut input_field = TextArea::default();

        input_field.set_block(Block::default().borders(Borders::ALL));

        Self { input_field }
    }
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Esc => break,
                _ => app.input_field.input_without_shortcuts(key),
            };
        }
    }

    Ok(())
}

fn ui(frame: &mut Frame, app: &mut App) {
    frame.render_widget(&app.input_field, frame.area());
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| run_app(terminal, &mut app))
}

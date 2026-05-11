use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Layout},
    widgets::{Block, List, ListItem},
};
use ratatui_textarea::TextArea;

struct App {
    input_field: TextArea<'static>,
    messages: Vec<String>,
}

impl App {
    fn new() -> Self {
        let mut input_field = TextArea::default();

        input_field.set_block(Block::bordered());

        Self {
            input_field,
            messages: Vec::new(),
        }
    }

    fn submit_text(&mut self) {
        let text = self.input_field.lines().join("");

        if !text.trim().is_empty() {
            self.messages.push(text);
            self.input_field.clear();
        }
    }
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Enter => app.submit_text(),
                _ => {
                    app.input_field.input_without_shortcuts(key);
                }
            };
        }
    }

    Ok(())
}

fn ui(frame: &mut Frame, app: &mut App) {
    let layout = Layout::vertical([Constraint::Max(3), Constraint::Fill(1)])
        .split(frame.area());

    let messages_list =
        List::new(app.messages.iter().map(|i| ListItem::new(i.as_str())))
            .block(Block::bordered());

    frame.render_widget(&app.input_field, layout[0]);
    frame.render_widget(messages_list, layout[1]);
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| run_app(terminal, &mut app))
}

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{Block, Clear, Paragraph},
};

#[derive(Default)]
struct App {
    show_popup: bool,
}

impl App {
    fn toggle_popup(&mut self) {
        self.show_popup = !self.show_popup;
    }
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('t') => app.toggle_popup(),
                _ => break,
            }
        }
    }
    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    let vert_layout =
        Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).split(frame.area());

    frame.render_widget(
        Paragraph::new("Window 1")
            .block(Block::bordered())
            .on_red(),
        vert_layout[0],
    );
    frame.render_widget(
        Paragraph::new("Window 2")
            .block(Block::bordered())
            .on_blue(),
        vert_layout[1],
    );

    if app.show_popup {
        let popup_area = frame
            .area()
            .centered(Constraint::Percentage(60), Constraint::Percentage(40));
        let popup_text = Paragraph::new("This is a popup!").block(Block::bordered());
        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup_text, popup_area);
    }
}

fn main() -> std::io::Result<()> {
    let mut app = App::default();
    ratatui::run(|terminal| run_app(terminal, &mut app))
}

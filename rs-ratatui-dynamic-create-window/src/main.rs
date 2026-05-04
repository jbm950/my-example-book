use ratatui::{
    DefaultTerminal,
    Frame,
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph}
};


struct App {
    show_side_win: bool,
}


impl App {
    fn new() -> Self {
        Self { show_side_win: false }
    }

    fn toggle_side_window(&mut self) {
        self.show_side_win = !self.show_side_win;
    }
}


fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('t') => app.toggle_side_window(),
                _ => break
            }
        }
    }
    Ok(())
}


fn ui(frame: &mut Frame, app: &App) {
    let bordered_block = Block::default().borders(Borders::ALL);

    let primary_text = Paragraph::new("Left window")
        .block(bordered_block.clone());

    if app.show_side_win {
        let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
            .split(frame.area());

        let side_text = Paragraph::new("Right Window")
            .block(bordered_block);

        frame.render_widget(primary_text, layout[0]);
        frame.render_widget(side_text, layout[1]);
    } else {
        frame.render_widget(primary_text, frame.area());
    }
}


fn main() -> std::io::Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| run_app(terminal, &mut app))
}

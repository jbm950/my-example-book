use ratatui::{
    DefaultTerminal,
    Frame,
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph}
};


enum ScreenVisible {
    Screen1,
    Screen2,
}


impl ScreenVisible {
    fn toggle(&mut self) {
        *self = match self {
            ScreenVisible::Screen1 => ScreenVisible::Screen2,
            ScreenVisible::Screen2 => ScreenVisible::Screen1,
        }
    }
}


struct App {
    screen: ScreenVisible,
}


impl App {
    fn new() -> Self {
        Self {screen: ScreenVisible::Screen1}
    }
}


fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('s') => app.screen.toggle(),
                _ => break
            }
        }
    }
    Ok(())
}


fn ui(frame: &mut Frame, app: &App) {
    match app.screen {
        ScreenVisible::Screen1 => split_screen(frame, Direction::Horizontal,
                                               ["Window 1", "Window 2"]),
        ScreenVisible::Screen2 => split_screen(frame, Direction::Vertical,
                                               ["Window 3", "Window 4"]),
    }
}


fn split_screen(frame: &mut Frame, direction: Direction, labels: [&str; 2]) {
    let layout = Layout::new(direction, [Constraint::Fill(1), Constraint::Fill(1)])
        .split(frame.area());

    for (&area, label) in layout.iter().zip(labels) {
        frame.render_widget(
            Paragraph::new(label)
                .block(Block::default().borders(Borders::ALL)),
            area
        );
    }
}


fn main() -> std::io::Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| run_app(terminal, &mut app))?;
    Ok(())
}

use ratatui::{
    DefaultTerminal,
    Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph}
};


fn run_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| { ui(frame) })?;

        if let Event::Key(_) = event::read()? {
            break
        }
    }
    Ok(())
}


fn ui(frame: &mut Frame) {
    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.area());

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(top_layout[1]);

    let win_1_text = Paragraph::new("Window 1")
        .block(Block::default().borders(Borders::ALL));
    let win_2_text = Paragraph::new("Window 2")
        .block(Block::default().borders(Borders::ALL));
    let win_3_text = Paragraph::new("Window 3")
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(win_1_text, top_layout[0]);
    frame.render_widget(win_2_text, right_layout[0]);
    frame.render_widget(win_3_text, right_layout[1]);
}


fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| { run_app(terminal) })?;
    Ok(())
}

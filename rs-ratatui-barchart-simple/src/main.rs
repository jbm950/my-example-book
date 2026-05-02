use ratatui::{
    DefaultTerminal,
    Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    widgets::{Bar, BarChart},
};


fn run_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame))?;

        if let Event::Key(_) = event::read()? {
            break
        }
    }
    Ok(())
}


fn ui(frame: &mut Frame) {
    let layout = Layout::new(Direction::Horizontal,
                             [Constraint::Fill(1), Constraint::Fill(1)])
        .spacing(3)
        .split(frame.area());

    frame.render_widget(
        BarChart::horizontal([
            Bar::with_label("Item 1", 3).text_value(""),
            Bar::with_label("Item 2", 4).text_value(""),
            Bar::with_label("Item 3", 1).text_value(""),
        ])
        .bar_width(1),
        layout[0],
    );

    frame.render_widget(
        BarChart::vertical([
            Bar::new(6).red(),
            Bar::new(5).blue(),
            Bar::new(3).green(),
        ])
        .bar_width(9),
        layout[1],
    );
}


fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| run_app(terminal))?;
    Ok(())
}

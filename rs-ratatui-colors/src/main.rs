use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

fn run_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(_) = event::read()? {
            break;
        }
    }
    Ok(())
}

fn ui(frame: &mut Frame) {
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(frame.area());

    let [rtop, rbottom] =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(right);

    let win_1_text = Paragraph::new("Window 1").block(
        Block::bordered()
            .style(Style::new().bg(Color::from_u32(0x181825)))
            .border_style(Style::new().fg(Color::from_u32(0xF06262))),
    );
    let win_2_text = Paragraph::new("Window 2").block(
        Block::bordered()
            .style(Style::new().bg(Color::from_u32(0x282C34)))
            .title("Title is Green!")
            .title_style(Style::new().fg(Color::from_u32(0x228B22))),
    );
    let win_3_text = Paragraph::new(Line::from(vec![
        Span::styled("This text is red. ", Style::new().fg(Color::Red)),
        Span::raw("This text has no formatting. "),
        Span::styled(
            "This text is bold and blue",
            Style::new()
                .fg(Color::from_u32(0x0000FF))
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .block(Block::bordered());

    frame.render_widget(win_1_text, left);
    frame.render_widget(win_2_text, rtop);
    frame.render_widget(win_3_text, rbottom);
}

fn main() -> std::io::Result<()> {
    ratatui::run(run_app)
}

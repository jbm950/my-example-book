use std::f64::consts::PI;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event,
    style::{Color, Style},
    symbols,
    widgets::{Axis, Block, Chart, Dataset, GraphType},
};

const NUM_POINTS: usize = 100;

fn run_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let step = 2.0 * PI / (NUM_POINTS - 1) as f64;
    let data: Vec<(f64, f64)> = (0..NUM_POINTS)
        .map(|i| {
            let x = i as f64 * step;
            (x, x.sin())
        })
        .collect();

    loop {
        terminal.draw(|frame| ui(frame, &data))?;

        if event::read()?.as_key_press_event().is_some() {
            break;
        }
    }

    Ok(())
}

fn ui(frame: &mut Frame, data: &[(f64, f64)]) {
    let dataset = Dataset::default()
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::new().fg(Color::Green))
        .data(data);

    let chart = Chart::new(vec![dataset])
        .block(Block::bordered().title("Sine Wave"))
        .x_axis(
            Axis::default()
                .title("x")
                .bounds([0.0, 2.0 * PI])
                .labels(["0", "π", "2π"]),
        )
        .y_axis(
            Axis::default()
                .title("sin(x)")
                .bounds([-1.0, 1.0])
                .labels(["-1", "0", "1"]),
        );

    frame.render_widget(chart, frame.area());
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal);

    ratatui::restore();

    result
}

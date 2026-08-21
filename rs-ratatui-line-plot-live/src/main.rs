use std::{collections::VecDeque, f64::consts::PI};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event,
    style::{Color, Style},
    symbols,
    widgets::{Axis, Block, Chart, Dataset, GraphType},
};
use tokio::{sync::mpsc, time::Duration};

const CHANNEL_CAPACITY: usize = 1;
const BUFFER_CAPACITY: usize = 200;
const NUM_POINTS: usize = 100;
const STEP: f64 = 2.0 * PI / (NUM_POINTS - 1) as f64;

struct Data {
    buffer: VecDeque<(f64, f64)>,
}

impl Data {
    fn new() -> Self {
        let mut buffer = VecDeque::with_capacity(BUFFER_CAPACITY);

        for i in 0..NUM_POINTS {
            let x = i as f64 * STEP;
            buffer.push_back((x, x.sin()));
        }

        Self {
            buffer,
        }
    }

    fn points(&self) -> Vec<(f64, f64)> {
        self.buffer.iter().copied().collect()
    }

    fn new_point(&mut self) {
        let (last_x, _) = self.buffer.back().unwrap();
        let new_x = last_x + STEP;

        if self.buffer.len() == BUFFER_CAPACITY {
            self.buffer.pop_front();
        }
        self.buffer.push_back((new_x, new_x.sin()));
    }

    fn x_bounds(&self) -> (f64, f64) {
        let (first_x, _) = self.buffer.front().unwrap();
        let (last_x, _) = self.buffer.back().unwrap();
        (*first_x, *last_x)
    }
}

fn key_events(tx: mpsc::Sender<()>) {
    while let Ok(key_event) = event::read() { 
        if key_event.as_key_press_event().is_some()
            && tx.blocking_send(()).is_err()
        {
            break;
        }
    }
}

async fn run_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut data = Data::new();

    let (key_tx, mut key_rx) = mpsc::channel(CHANNEL_CAPACITY);

    let mut update_interval = tokio::time::interval(Duration::from_millis(10));
    let mut render_interval = tokio::time::interval(Duration::from_millis(33)); // 30 fps
    render_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    std::thread::spawn(move || key_events(key_tx));

    loop {
        tokio::select! {
            _ = key_rx.recv() => {
                break;
            }

            _ = update_interval.tick() => {
                data.new_point();
            }

            _ = render_interval.tick() => {
                terminal.draw(|frame| ui(frame, &data))?;
            }
        }
    }

    Ok(())
}

fn ui(frame: &mut Frame, data: &Data) {
    let points = data.points();

    let dataset = Dataset::default()
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::new().fg(Color::Green))
        .data(&points);

    let chart = Chart::new(vec![dataset])
        .block(Block::bordered().title("Sine Wave"))
        .x_axis(
            Axis::default()
                .title("x")
                .bounds(data.x_bounds().into())
        )
        .y_axis(
            Axis::default()
                .title("sin(x)")
                .bounds([-1.0, 1.0])
                .labels(["-1", "0", "1"]),
        );

    frame.render_widget(chart, frame.area());
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal).await;

    ratatui::restore();

    result
}

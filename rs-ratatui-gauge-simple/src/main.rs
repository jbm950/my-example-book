use std::time::Duration;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Layout},
    style::Style,
    widgets::{Block, Gauge, LineGauge, Paragraph},
};
use tokio::sync::mpsc;

#[derive(Clone, Copy)]
enum Message {
    Increment,
    Reset,
    Exit,
}

#[derive(Default)]
struct App {
    count: u16,
    exit: bool,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.count = (self.count + 7).min(100);
            }
            Message::Reset => self.count = 0,
            Message::Exit => self.exit = true,
        }
    }
}

fn key_events(tx: mpsc::Sender<Message>) -> std::io::Result<()> {
    loop {
        if let Some(key) = event::read()?.as_key_press_event() {
            let message = match key.code {
                KeyCode::Char('r') => Message::Reset,
                _ => Message::Exit,
            };

            let exit = matches!(message, Message::Exit);
            if tx.blocking_send(message).is_err() || exit {
                return Ok(());
            }
        }
    }
}

async fn worker(tx: mpsc::Sender<Message>) {
    let mut interval = tokio::time::interval(Duration::from_millis(300));
    interval.tick().await;  // Allow first frame to be the initial state

    loop {
        interval.tick().await;
        if tx.send(Message::Increment).await.is_err() {
            return;
        }
    }
}

async fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    let capacity = 32;
    let (tx, mut rx) = mpsc::channel(capacity);

    let input_tx = tx.clone();
    let _ = std::thread::spawn(move || key_events(input_tx));
    tokio::spawn(worker(tx));

    terminal.draw(|frame| ui(frame, app))?;

    while let Some(message) = rx.recv().await {
        app.update(message);

        terminal.draw(|frame| ui(frame, app))?;

        if app.exit {
            break;
        }
    }

    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    let bordered_block = Block::bordered().title("Gauges Example");
    frame.render_widget(&bordered_block, frame.area());

    let inner_area = bordered_block.inner(frame.area());
    let [text_area, gauge_area, line_gauge_area] =
        Layout::vertical([Constraint::Length(1); 3]).areas(inner_area);

    let count_text = Paragraph::new(format!("Count: {}", app.count));
    frame.render_widget(count_text, text_area);

    let gauge = Gauge::default().label("Gauge").percent(app.count);
    frame.render_widget(gauge, gauge_area);

    let line_gauge = LineGauge::default()
        .label("Line Gauge")
        .ratio(app.count as f64 / 100.0)
        .filled_style(Style::new().white().on_red());
    frame.render_widget(line_gauge, line_gauge_area);
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    let result = run_app(&mut terminal, &mut app).await;

    ratatui::restore();

    result
}

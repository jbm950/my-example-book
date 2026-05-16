use std::time::Duration;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
};
use tokio::sync::mpsc;

#[derive(Clone, Copy)]
enum Message {
    Increment,
    Decrement,
    ToggleSideWindow,
    Exit,
}

#[derive(Default)]
struct App {
    count: i16,
    show_side_win: bool,
    exit: bool,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
            Message::ToggleSideWindow => self.show_side_win = !self.show_side_win,
            Message::Exit => self.exit = true,
        }
    }
}

fn key_events(tx: mpsc::Sender<Message>) -> std::io::Result<()> {
    loop {
        if let Some(key) = event::read()?.as_key_press_event() {
            let message = match key.code {
                KeyCode::Char('t') => Message::ToggleSideWindow,
                _ => Message::Exit,
            };

            if tx.blocking_send(message).is_err() {
                return Ok(());
            }

            if matches!(message, Message::Exit) {
                return Ok(());
            }
        }
    }
}

async fn worker(tx: mpsc::Sender<Message>) {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    interval.tick().await;

    let messages = [Message::Increment, Message::Increment, Message::Decrement];
    for message in messages.into_iter().cycle() {
        interval.tick().await;
        if tx.send(message).await.is_err() {
            return;
        }
    }
}

async fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    let capacity = 32;
    let (tx, mut rx) = mpsc::channel(capacity);

    let input_tx = tx.clone();
    std::thread::spawn(move || key_events(input_tx));
    tokio::spawn(worker(tx.clone()));

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
    let bordered_block = Block::default().borders(Borders::ALL);

    let primary_text = Paragraph::new(app.count.to_string()).block(bordered_block.clone());

    if app.show_side_win {
        let [left, right] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(frame.area());

        let side_text = Paragraph::new("Right Window").block(bordered_block);

        frame.render_widget(primary_text, left);
        frame.render_widget(side_text, right);
    } else {
        frame.render_widget(primary_text, frame.area());
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    let result = run_app(&mut terminal, &mut app).await;

    ratatui::restore();

    result
}

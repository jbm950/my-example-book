use std::time::Duration;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
};
use tokio::sync::mpsc;

enum UpdateCount {
    Increment,
    Decrement,
}

struct App {
    count: i16,
    show_side_win: bool,
}

impl App {
    fn new() -> Self {
        Self {
            count: 0,
            show_side_win: false,
        }
    }

    fn toggle_side_window(&mut self) {
        self.show_side_win = !self.show_side_win;
    }

    fn update_count(&mut self, message: UpdateCount) {
        match message {
            UpdateCount::Increment => self.count += 1,
            UpdateCount::Decrement => self.count -= 1,
        }
    }
}

async fn worker(tx: mpsc::Sender<UpdateCount>) {
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    interval.tick().await;

    loop {
        interval.tick().await;
        let _ = tx.send(UpdateCount::Increment).await;

        interval.tick().await;
        let _ = tx.send(UpdateCount::Increment).await;

        interval.tick().await;
        let _ = tx.send(UpdateCount::Decrement).await;
    }
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    let capacity = 32;
    let (tx, mut rx) = mpsc::channel(capacity);

    tokio::spawn(worker(tx));

    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('t') => app.toggle_side_window(),
                    _ => break,
                }
            }
        }

        while let Ok(message) = rx.try_recv() {
            app.update_count(message);
        }
    }
    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    let bordered_block = Block::default().borders(Borders::ALL);

    let primary_text = Paragraph::new(app.count.to_string()).block(bordered_block.clone());

    if app.show_side_win {
        let layout =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).split(frame.area());

        let side_text = Paragraph::new("Right Window").block(bordered_block);

        frame.render_widget(primary_text, layout[0]);
        frame.render_widget(side_text, layout[1]);
    } else {
        frame.render_widget(primary_text, frame.area());
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| run_app(terminal, &mut app))
}

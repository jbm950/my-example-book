use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self},
    layout::Constraint,
    widgets::{Block, Clear, List, ListItem, Paragraph},
};
use reqwest::Client;
use serde_json::Value;
use tokio::sync::mpsc;

enum Message {
    ReposLoaded(Value),
    Error(String),
    Exit,
}

#[derive(Default)]
struct App {
    gitea_data: Option<Value>,
    error: Option<String>,
    exit: bool,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::ReposLoaded(repo_data) => {
                self.gitea_data = Some(repo_data);
                self.error = None
            }
            Message::Error(err) => self.error = Some(err),
            Message::Exit => self.exit = true,
        }
    }
}

fn key_events(tx: mpsc::Sender<Message>) -> std::io::Result<()> {
    loop {
        if let Some(_) = event::read()?.as_key_press_event() {
            let _ = tx.blocking_send(Message::Exit);
            return Ok(());
        }
    }
}

async fn fetch_repos(tx: mpsc::Sender<Message>) {
    let result: Result<Value, String> = async {
        let token =
            std::fs::read_to_string("creds").map_err(|e| format!("Failed to read token: {e}"))?;

        let client = Client::new();
        let response = client
            .get("http://gitea.odin.orlfl.milamnet.io/api/v1/repos/search")
            .bearer_auth(token.trim())
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?
            .error_for_status()
            .map_err(|e| format!("Server returned error: {e}"))?;

        let response = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON response: {e}"))?;

        Ok(response)
    }
    .await;

    let message = match result {
        Ok(repo_data) => Message::ReposLoaded(repo_data),
        Err(err) => Message::Error(err),
    };

    if tx.send(message).await.is_err() {
        return;
    }
}

async fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    let capacity = 32;
    let (tx, mut rx) = mpsc::channel(capacity);

    let input_tx = tx.clone();
    std::thread::spawn(move || key_events(input_tx));
    tokio::spawn(fetch_repos(tx));

    loop {
        terminal.draw(|frame| ui(frame, app))?;

        let Some(message) = rx.recv().await else {
            break;
        };

        app.update(message);

        if app.exit {
            break;
        }
    }

    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    if let Some(gitea_data) = &app.gitea_data {
        let repo_names = List::new(
            gitea_data["data"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(|i| i["full_name"].as_str())
                .map(ListItem::new),
        )
        .block(Block::bordered());
        frame.render_widget(repo_names, frame.area())
    } else {
        let not_loaded_message = Paragraph::new("Still loading data").block(Block::bordered());
        frame.render_widget(not_loaded_message, frame.area());
    }

    if let Some(err) = &app.error {
        let popup_area = frame
            .area()
            .centered(Constraint::Percentage(20), Constraint::Percentage(10));
        let popup_text = Paragraph::new(err.to_string())
            .block(Block::bordered())
            .centered();
        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup_text, popup_area);
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

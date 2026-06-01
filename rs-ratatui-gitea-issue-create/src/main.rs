use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Clear, Paragraph, Widget},
};
use ratatui_textarea::TextArea;
use reqwest::Client;
use serde::Serialize;
use tokio::sync::mpsc;

#[derive(Serialize)]
struct Issue {
    title: String,
    body: String,
}

impl Issue {
    fn new(title: String, body: String) -> Self {
        Self { title, body }
    }
}

enum Message {
    Key(KeyEvent),
    IssueCreated(String),
    Error(String),
}

enum ActiveField {
    Title,
    Body,
}

impl ActiveField {
    fn toggle(&mut self) {
        *self = match self {
            Self::Title => Self::Body,
            Self::Body => Self::Title,
        }
    }
}

#[derive(PartialEq)]
enum Mode {
    Normal,
    Edit,
}

struct InputField {
    title: &'static str,
    text_area: TextArea<'static>,
    base_style: Style,
    multiline: bool,
}

impl InputField {
    fn new(title: &'static str, multiline: bool) -> Self {
        Self {
            title,
            text_area: TextArea::default(),
            base_style: Style::default().bg(Color::from_u32(0x181825)),
            multiline,
        }
    }

    fn value(&self) -> String {
        let separator = if self.multiline { "\n" } else { "" };
        self.text_area.lines().join(separator)
    }

    fn clear(&mut self) {
        self.text_area.clear();
    }

    fn set_style_inactive(&mut self) {
        self.text_area.set_block(
            Block::bordered()
                .title(self.title)
                .style(self.base_style)
                .border_type(BorderType::Thick),
        );
        self.text_area.set_cursor_style(Style::default());
        self.text_area.set_cursor_line_style(Style::default());
    }

    fn set_style_active(&mut self) {
        self.text_area.set_block(
            Block::bordered()
                .title(self.title)
                .style(self.base_style)
                .border_style(Style::default().fg(Color::from_u32(0xFAB387)))
                .border_type(BorderType::Thick),
        );
    }

    fn set_style_edit(&mut self) {
        self.text_area
            .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    }

    fn input(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Enter && !self.multiline {
            return;
        }
        self.text_area.input_without_shortcuts(key);
    }
}

impl Widget for &InputField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.text_area.render(area, buf);
    }
}

enum Popup {
    Success(String),
    Error(String),
}

impl Widget for &Popup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let message = match self {
            Popup::Success(msg) => format!("Success: {}", msg),
            Popup::Error(msg) => format!("Error: {}", msg),
        };

        let popup_area = area.centered(Constraint::Percentage(30), Constraint::Percentage(15));

        Clear.render(popup_area, buf);
        Paragraph::new(message)
            .block(Block::bordered().style(Style::default().bg(Color::from_u32(0x181825))))
            .centered()
            .render(popup_area, buf);
    }
}

struct App {
    active_field: ActiveField,
    events_tx: mpsc::Sender<Message>,
    title_field: InputField,
    body_field: InputField,
    mode: Mode,
    popup: Option<Popup>,
    exit: bool,
}

impl App {
    pub fn new(events_tx: mpsc::Sender<Message>) -> Self {
        Self {
            active_field: ActiveField::Title,
            events_tx,
            title_field: InputField::new("Title", false),
            body_field: InputField::new("Body", true),
            mode: Mode::Normal,
            popup: None,
            exit: false,
        }
    }

    fn update(&mut self, message: Message) {
        // Dismiss popups on every new event, whatever the source
        self.popup = None;

        match message {
            Message::IssueCreated(msg) => self.popup = Some(Popup::Success(msg)),
            Message::Error(err) => self.popup = Some(Popup::Error(err)),
            other => match self.mode {
                Mode::Edit => self.update_edit(other),
                Mode::Normal => self.update_normal(other),
            },
        }
    }

    fn update_edit(&mut self, message: Message) {
        match message {
            Message::Key(key) => {
                let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

                match key.code {
                    KeyCode::Char('c') if ctrl => self.mode = Mode::Normal,
                    KeyCode::Esc => self.mode = Mode::Normal,
                    _ => {
                        self.active_field_mut().input(key);
                    }
                }
            }
            _ => {}
        }
    }

    fn update_normal(&mut self, message: Message) {
        match message {
            Message::Key(key) => match key.code {
                KeyCode::Tab => self.active_field.toggle(),
                KeyCode::Char('i') => self.mode = Mode::Edit,
                KeyCode::Char('j') => self.active_field = ActiveField::Body,
                KeyCode::Char('k') => self.active_field = ActiveField::Title,
                KeyCode::Enter => self.submit_issue(),
                KeyCode::Esc => self.exit = true,
                _ => {}
            },
            _ => {}
        }
    }

    fn active_field_mut(&mut self) -> &mut InputField {
        match self.active_field {
            ActiveField::Title => &mut self.title_field,
            ActiveField::Body => &mut self.body_field,
        }
    }

    fn build_issue(&self) -> Result<Issue, String> {
        let title = self.title_field.value();
        let body = self.body_field.value();

        if title.is_empty() || body.is_empty() {
            return Err(String::from("One or more required fields were empty"));
        }

        Ok(Issue::new(title, body))
    }

    fn submit_issue(&mut self) {
        match self.build_issue() {
            Ok(issue) => {
                tokio::spawn(create_issue(self.events_tx.clone(), issue));

                self.title_field.clear();
                self.body_field.clear();
            }
            Err(msg) => {
                self.popup = Some(Popup::Error(msg));
            }
        }
    }
}

fn key_events(tx: mpsc::Sender<Message>) -> std::io::Result<()> {
    loop {
        if let Some(key) = event::read()?.as_key_press_event() {
            if tx.blocking_send(Message::Key(key)).is_err() {
                return Ok(());
            }
        }
    }
}

async fn create_issue(tx: mpsc::Sender<Message>, issue: Issue) {
    let result: Result<(), String> = async {
        let token =
            std::fs::read_to_string("creds").map_err(|e| format!("Failed to read token: {e}"))?;

        let client = Client::new();

        client
            .post("http://gitea.odin.orlfl.milamnet.io/api/v1/repos/jmilam/test-repo/issues")
            .bearer_auth(token.trim())
            .json(&issue)
            .send()
            .await
            .map_err(|e| format!("Request failed: {e}"))?
            .error_for_status()
            .map_err(|e| format!("Server returned error: {e}"))?;

        Ok(())
    }
    .await;

    let message = match result {
        Ok(_) => Message::IssueCreated(String::from("Issue created successfully")),
        Err(err) => Message::Error(err),
    };

    if tx.send(message).await.is_err() {
        return;
    }
}

async fn run_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let capacity = 32;
    let (tx, mut rx) = mpsc::channel(capacity);

    let input_tx = tx.clone();
    std::thread::spawn(move || key_events(input_tx));
    let mut app = App::new(tx);

    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

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

fn ui(frame: &mut Frame, app: &mut App) {
    let [title_area, body_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(frame.area());

    app.title_field.set_style_inactive();
    app.body_field.set_style_inactive();

    app.active_field_mut().set_style_active();
    if app.mode == Mode::Edit {
        app.active_field_mut().set_style_edit();
    }

    frame.render_widget(&app.title_field, title_area);
    frame.render_widget(&app.body_field, body_area);

    if let Some(popup) = &app.popup {
        frame.render_widget(popup, frame.area());
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal).await;

    ratatui::restore();

    result
}

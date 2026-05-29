use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType},
};
use ratatui_textarea::TextArea;

enum ActiveField {
    Field1,
    Field2,
}

#[derive(PartialEq)]
enum Mode {
    Edit,
    Normal,
}

struct App {
    active_field: ActiveField,
    mode: Mode,
    input_field_1: TextArea<'static>,
    input_field_2: TextArea<'static>,
}

impl App {
    fn new() -> Self {
        Self {
            active_field: ActiveField::Field1,
            mode: Mode::Normal,
            input_field_1: TextArea::default(),
            input_field_2: TextArea::default(),
        }
    }

    fn active_field_mut(&mut self) -> &mut TextArea<'static> {
        match self.active_field {
            ActiveField::Field1 => &mut self.input_field_1,
            ActiveField::Field2 => &mut self.input_field_2,
        }
    }

    fn toggle_active_field(&mut self) {
        self.active_field = match self.active_field {
            ActiveField::Field1 => ActiveField::Field2,
            ActiveField::Field2 => ActiveField::Field1,
        }
    }

    fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Normal => Mode::Edit,
            Mode::Edit => Mode::Normal,
        }
    }
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Esc => break,
                _ => {}
            }

            match app.mode {
                Mode::Normal => handle_normal_mode(app, key),
                Mode::Edit => handle_edit_mode(app, key),
            }
        }
    }

    Ok(())
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('k') => app.active_field = ActiveField::Field1,
        KeyCode::Char('j') => app.active_field = ActiveField::Field2,
        KeyCode::Char('i') => app.toggle_mode(),
        KeyCode::Tab => app.toggle_active_field(),
        _ => {}
    }
}

fn handle_edit_mode(app: &mut App, key: KeyEvent) {
    if key.code == KeyCode::Char('c')
        && key.modifiers.contains(KeyModifiers::CONTROL)
    {
        app.toggle_mode();
        return;
    }
    app.active_field_mut().input_without_shortcuts(key);
}

fn field_style() -> Style {
    Style::default().bg(Color::from_u32(0x181825))
}

fn ui(frame: &mut Frame, app: &mut App) {
    let [field_area_1, field_area_2] =
        Layout::vertical([Constraint::Percentage(30), Constraint::Percentage(70)])
            .areas(frame.area());

    set_inactive_field_style(&mut app.input_field_1);
    set_inactive_field_style(&mut app.input_field_2);

    app.active_field_mut().set_block(
        Block::bordered()
            .style(field_style())
            .border_style(Style::default().fg(Color::from_u32(0xFAB387)))
            .border_type(BorderType::Thick),
    );
    if app.mode == Mode::Edit {
        app.active_field_mut()
            .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    }

    frame.render_widget(&app.input_field_1, field_area_1);
    frame.render_widget(&app.input_field_2, field_area_2);
}

fn set_inactive_field_style(field: &mut TextArea<'static>) {
    field.set_block(
        Block::bordered()
            .border_type(BorderType::Thick)
            .style(field_style()),
    );
    field.set_cursor_style(Style::default());
    field.set_cursor_line_style(Style::default());
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| run_app(terminal, &mut app))
}

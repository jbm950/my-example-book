use ratatui::{
    DefaultTerminal,
    Frame,
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Direction, Layout},
    style::Modifier,
    widgets::{List, ListItem, ListState, Paragraph}
};


struct App {
    list_state: ListState,
    list_items: Vec<&'static str>,
}


impl App {
    fn new() -> Self {
        Self {
            list_state: ListState::default().with_selected(Some(0)),
            list_items: vec!("Item 1", "Item 2", "Item 3", "Item 4"),
        }
    }
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('j') => app.list_state.select_next(),
                KeyCode::Char('k') => app.list_state.select_previous(),
                _ => break
            }
        }
    }

    Ok(())
}


fn ui(frame: &mut Frame, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Fill(1)])
        .split(frame.area());

    let my_list = List::new(
        app.list_items
            .iter()
            .map(|&i| ListItem::new(i))
    )
        .highlight_style(Modifier::REVERSED);
    frame.render_stateful_widget(my_list, layout[0], &mut app.list_state);

    if let Some(selected) = app.list_state.selected() {
        frame.render_widget(
            Paragraph::new(app.list_items[selected]),
            layout[1]
        );
    }
}


fn main() -> std::io::Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| { run_app(terminal, &mut app) })?;
    Ok(())
}

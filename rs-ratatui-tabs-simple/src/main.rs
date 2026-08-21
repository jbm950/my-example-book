use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyCode},
    layout::{Rect},
    style::Style,
    symbols,
    widgets::{Block, Paragraph, Tabs},
};

struct App {
    titles: Vec<&'static str>,
    selected_tab: usize,
}

impl App {
    fn next_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % self.titles.len();
    }

    fn previous_tab(&mut self) {
        // equivalent to subtracting 1
        self.selected_tab = (self.selected_tab + self.titles.len() - 1) % self.titles.len();
    }
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    break Ok(());
                }
                KeyCode::Char('h') | KeyCode::Left => {
                    app.previous_tab();
                }
                KeyCode::Char('l') | KeyCode::Right => {
                    app.next_tab();
                }
                _ => {}
            }
        }
    }
}

fn ui(frame: &mut Frame, app: &App) {
    // Draw the tab bar on top of the block's top border, inset by 1
    // column on each side so it doesn't overwrite the corner glyphs.
    let tabs_row = Rect {
        x: frame.area().x + 1,
        y: frame.area().y,
        width: frame.area().width.saturating_sub(2),
        height: 1,
    };

    let tabs = Tabs::new(app.titles.iter().copied())
        .style(Style::new().white())
        .highlight_style(Style::new().red().on_black().bold())
        .select(app.selected_tab)
        .divider(symbols::DOT)
        .padding(" ", " ");

    let text = match app.selected_tab {
        0 => "First tab's text",
        1 => "Second tab also has text",
        2 => "Definitely tab 3's text",
        _ => unreachable!(),
    };
    let paragraph = Paragraph::new(text).block(Block::bordered());

    // Draw the tab bar on top of the block's top border, inset by 1
    // column on each side so it doesn't overwrite the corner glyphs.
    frame.render_widget(paragraph, frame.area());
    frame.render_widget(tabs, tabs_row);
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App { titles: vec!["Tab 1", "Tab 2", "Tab 3"], selected_tab: 0 };

    let result = run_app(&mut terminal, &mut app);

    ratatui::restore();

    result
}

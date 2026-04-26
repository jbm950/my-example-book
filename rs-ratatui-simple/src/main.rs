use ratatui::{DefaultTerminal, Frame, widgets::Paragraph};


struct App {
    pub my_hello_text: String,
}


impl App {
    fn new() -> App {
        App {
            my_hello_text: String::from("Hello World"),
        }
    }
}


fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut app = App::new();
    ratatui::run(|terminal| {run_app(terminal, &mut app)})?;
    Ok(())
}


fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        // Redraw each frame
        terminal.draw(|frame| {ui(frame, app)})?;

        // Event Loop
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}


fn ui(frame: &mut Frame, app: &App) {
    let hello_widget = Paragraph::new(app.my_hello_text.clone());
    frame.render_widget(hello_widget, frame.area());
}

use ratatui::{
    DefaultTerminal,
    Frame,
    crossterm::event::{self, Event},
    widgets::{List},
};


fn run_app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| { ui(frame) })?;

        if let Event::Key(_) = event::read()? {
            break
        }
    }
    Ok(())
}


fn ui(frame: &mut Frame) {
    let my_list = List::new(["Item 1", "Item 2", "Item 3", "Item 4"]);
    frame.render_widget(my_list, frame.area());
}


fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| { run_app(terminal) })?;
    Ok(())
}

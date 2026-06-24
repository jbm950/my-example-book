use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event,
    layout::Constraint,
    style::Style,
    widgets::{Row, Table},
};

struct Data {
    name: String,
    role: String,
    id: u32,
    qty: u32,
}

struct App {
    data: Vec<Data>,
}

fn run_app(terminal: &mut DefaultTerminal, app: App) -> std::io::Result<()> {

    loop {
        terminal.draw(|frame| ui(frame, &app))?;

        if event::read()?.as_key_press_event().is_some() {
            break;
        }
    }

    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    let header = Row::new(["Name", "Role", "ID", "QTY"]).style(Style::new().bold());

    let rows = app.data.iter().map(|data| {
        Row::new([
            data.name.clone(),
            data.role.clone(),
            data.id.to_string(),
            data.qty.to_string(),
        ])
    });

    const WIDTHS: [Constraint; 4] = [
        Constraint::Percentage(30),
        Constraint::Percentage(30),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ];
    let table = Table::new(rows, WIDTHS).header(header).column_spacing(1);

    frame.render_widget(table, frame.area());
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let data = vec![
        Data {
            name: "Steve Irwin".into(),
            role: "Actor".into(),
            id: 1,
            qty: 2,
        },
        Data {
            name: "Darth Vader".into(),
            role: "Villian".into(),
            id: 2,
            qty: 5,
        },
        Data {
            name: "Pikachu".into(),
            role: "Pokemon".into(),
            id: 3,
            qty: 15,
        },
        Data {
            name: "Ember".into(),
            role: "Dog".into(),
            id: 4,
            qty: 1,
        },
    ];
    let app = App { data };
    let result = run_app(&mut terminal, app);

    ratatui::restore();

    result
}

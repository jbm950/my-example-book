use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::Modifier,
    text::Line,
    widgets::{Block, List, Paragraph},
};

use crate::{
    app::state::App,
    devices::{gps::GpsTelemetry, power::PowerTelemetry},
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let [power_area, gps_area] =
        Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(frame.area());

    let [power_telem_area, power_cmd_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(power_area);

    let power_window = Paragraph::new(power_lines(&app.power_telemetry)).block(Block::bordered());
    frame.render_widget(power_window, power_telem_area);

    let power_commands = List::new(app.power_commands.list_commands())
        .block(Block::bordered())
        .highlight_style(Modifier::REVERSED);
    frame.render_stateful_widget(
        power_commands,
        power_cmd_area,
        &mut app.power_commands.state(),
    );

    let gps_window = Paragraph::new(gps_lines(&app.gps_telemetry)).block(Block::bordered());
    frame.render_widget(gps_window, gps_area);
}

fn power_lines(power_telemetry: &Option<PowerTelemetry>) -> Vec<Line<'static>> {
    match power_telemetry {
        Some(val) => vec![
            Line::from("Power Telemetry Received"),
            Line::from(format!("Power Mode: {:?}", val.mode)),
            Line::from(format!("Charge %: {}", val.charge_percent)),
            Line::from(format!("Temp (deg C): {}", val.temperature_c)),
            Line::from(format!("Fault: {:?}", val.fault)),
        ],
        None => vec![Line::from("Waiting for Power Telemetry")],
    }
}

fn gps_lines(gps_telemetry: &Option<GpsTelemetry>) -> Vec<Line<'static>> {
    match gps_telemetry {
        Some(val) => vec![
            Line::from("GPS Telemetry Received"),
            Line::from(format!(
                "Time Week: {}, Seconds: {}",
                val.time.week, val.time.seconds_of_week
            )),
            Line::from("Position:"),
            Line::from(format!("    x: {}", val.position.x)),
            Line::from(format!("    y: {}", val.position.y)),
            Line::from(format!("    z: {}", val.position.z)),
            Line::from("Velocity:"),
            Line::from(format!("    x: {}", val.velocity.x)),
            Line::from(format!("    y: {}", val.velocity.y)),
            Line::from(format!("    z: {}", val.velocity.z)),
        ],
        None => vec![Line::from("Waiting for GPS Telemetry")],
    }
}

use ratatui::crossterm::event;
use tokio::sync::mpsc;

pub fn key_events(tx: mpsc::Sender<event::KeyEvent>) -> std::io::Result<()> {
    loop {
        if let Some(key_event) = event::read()?.as_key_press_event() {
            if tx.blocking_send(key_event).is_err() {
                break;
            }
        }
    }

    Ok(())
}

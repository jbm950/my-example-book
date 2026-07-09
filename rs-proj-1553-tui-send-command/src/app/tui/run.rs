use tokio::sync::mpsc;

use crate::{
    app::{
        state::App,
        tui::{key_events, ui::ui},
    },
    protocol::Transaction,
};

pub async fn run_app(
    app: &mut App,
    transactions_rx: &mut mpsc::Receiver<Transaction>,
) -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    let capacity = 32;
    let (input_tx, mut input_rx) = mpsc::channel(capacity);
    std::thread::spawn(move || key_events(input_tx));

    loop {
        terminal.draw(|frame| ui(frame, app))?;

        tokio::select! {
            Some(key_event) = input_rx.recv() => {
                app.handle_key(key_event).await;
            }

            Some(transaction) = transactions_rx.recv() => {
                app.handle_transaction(transaction);
            }
        }

        if app.exit {
            break;
        }
    }

    ratatui::restore();

    Ok(())
}

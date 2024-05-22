use std::io::Result;

use crossterm::event::{self, Event, KeyCode};

use crate::app::{App, AppState};

pub fn handle_events(app: &mut App) -> Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Release {
            return Ok(());
        }

        if key.code == KeyCode::Char('q') {
            app.app_state = AppState::Exit;
        }
    }
    Ok(())
}

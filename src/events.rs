use std::io::Result;

use crossterm::event::{self, Event, KeyCode};

use crate::app::{App, AppState};

pub fn handle_events(app: &mut App) -> Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Release {
            return Ok(());
        }

        match key.code {
            KeyCode::Esc => {
                app.app_state = AppState::Exit;
            }
            KeyCode::Backspace if !app.active_chat.typing_message.is_empty() => {
                app.active_chat.typing_message.pop();
            }
            KeyCode::Enter => {
                app.send_message(&app.active_chat.typing_message);
                app.active_chat.typing_message = String::new();
            }
            KeyCode::Char(value) => {
                app.active_chat.typing_message.push(value);
            }
            _ => {}
        }
    }
    Ok(())
}

use std::io::Result;

use crossterm::event::{self, Event, KeyCode};

use crate::app::{App, AppState};

pub fn handle_events(app: &mut App) -> Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Release {
            return Ok(());
        }

        // let active_chat = &app.chats[app.active_chat.unwrap()];
        let typing_message = &mut app.chats[app.active_chat].typing_message;
        match key.code {
            KeyCode::Esc => {
                app.app_state = AppState::Exit;
            }
            KeyCode::Backspace if !typing_message.is_empty() => {
                typing_message.pop();
            }
            KeyCode::Enter => {
                let message = typing_message.clone();
                typing_message.clear();
                app.send_message(message, app.active_chat);
            }
            KeyCode::Char(value) => {
                typing_message.push(value);
            }
            _ => {}
        }
    }
    Ok(())
}

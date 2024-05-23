use std::io::Result;

use crossterm::event::{self, Event, KeyCode};

use crate::app::{App, AppState, Message, MessageType};

pub async fn handle_events(app: &mut App) -> Result<()> {
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
                let message = Message {
                    id: "".into(),
                    time: "".into(),
                    author: "You".into(),
                    text: typing_message.clone(),
                    message_type: MessageType::Normal,
                };
                typing_message.clear();

                app.send_message(message).await;
            }
            KeyCode::Char(value) => {
                typing_message.push(value);
            }
            _ => {}
        }
    }
    Ok(())
}

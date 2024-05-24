use std::io::Result;

use crossterm::event::{self, Event, KeyCode};

use crate::app::{App, AppState, Message, MessageType};

pub async fn handle_events(app: &mut App) -> Result<()> {
    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Release {
            return Ok(());
        }

        let visible_messages = app.chats[app.active_chat].visible_messages;
        let total_messages = app.chats[app.active_chat].messages.len();
        let typing_message = &mut app.chats[app.active_chat].typing_message;
        match key.code {
            KeyCode::Esc => {
                app.app_state = AppState::Exit;
            }

            KeyCode::PageUp if visible_messages.is_some() => {
                let current_offset = app.chats[app.active_chat].chat_list_state.offset();
                if current_offset + visible_messages.unwrap() < total_messages {
                    *app.chats[app.active_chat].chat_list_state.offset_mut() = current_offset + 1;
                }
            }
            KeyCode::PageDown => {
                let current_offset = app.chats[app.active_chat].chat_list_state.offset();
                *app.chats[app.active_chat].chat_list_state.offset_mut() =
                    current_offset.saturating_sub(1);
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

use std::io::Result;

use crossterm::event::{self, Event, KeyCode};

use crate::app::{App, AppState, Message, MessageType};

pub async fn handle_events(app: &mut App<'_>) -> Result<()> {
    let chat = &mut app.chats[app.active_chat];

    if let Event::Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Release {
            return Ok(());
        }

        match key.code {
            KeyCode::Esc => {
                app.app_state = AppState::Exit;
            }

            KeyCode::PageUp if chat.visible_messages.is_some() => {
                let current_offset = chat.chat_list_state.offset();
                if current_offset + chat.visible_messages.unwrap() < chat.messages.len() {
                    *app.chats[app.active_chat].chat_list_state.offset_mut() = current_offset + 1;
                }
            }
            KeyCode::PageDown => {
                let current_offset = app.chats[app.active_chat].chat_list_state.offset();
                *app.chats[app.active_chat].chat_list_state.offset_mut() =
                    current_offset.saturating_sub(1);
            }

            KeyCode::Backspace if !chat.typing_message.is_empty() => {
                chat.typing_message.pop();
            }
            KeyCode::Enter if !chat.typing_message.is_empty() => {
                let message = Message {
                    id: "".into(),
                    time: "".into(),
                    author: "You".into(),
                    text: chat.typing_message.clone(),
                    message_type: MessageType::Normal,
                };
                chat.typing_message.clear();

                chat.send_message(message).await;
            }
            KeyCode::Char(value) => {
                chat.typing_message.push(value);
            }
            _ => {}
        }
    }
    Ok(())
}

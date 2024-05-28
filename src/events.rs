use std::io::Result;

use crossterm::event::{self, Event, KeyCode, MouseEventKind};
use ratatui::layout::Position;

use crate::{
    app::{App, AppState, Message, MessageType},
    ui::MessageStyle,
};

pub async fn handle_events(app: &mut App<'_>) -> Result<bool> {
    let event = event::read()?;
    let chat = &mut app.chats[app.active_chat];

    match event {
        // Only listen to key presses (not releases)
        Event::Key(key) if key.kind != event::KeyEventKind::Release => match key.code {
            KeyCode::Esc => {
                app.app_state = AppState::Exit;
            }

            KeyCode::PageUp | KeyCode::Up => {
                chat.scroll_up();
            }
            KeyCode::PageDown | KeyCode::Down => {
                chat.scroll_down();
            }

            KeyCode::Backspace if !chat.ui_state.typing_message.is_empty() => {
                chat.ui_state.typing_message.pop();
            }
            KeyCode::Enter if !chat.ui_state.typing_message.is_empty() => {
                let message = Message {
                    id: "".into(),
                    time: "".into(),
                    author: "You".into(),
                    text: chat.ui_state.typing_message.clone(),
                    message_type: MessageType::Normal,
                    lines: None,
                    style: MessageStyle::default(),
                };
                chat.ui_state.typing_message.clear();

                chat.send_message(message).await;

                // Bring scrolled chat back down to show sent message
                *app.chats[app.active_chat]
                    .ui_state
                    .chat_list_state
                    .offset_mut() = 0;

                // Set state changed; Reloads UI
                return Ok(true);
            }
            KeyCode::Char(value) => {
                chat.ui_state.typing_message.push(value);
            }
            _ => {}
        },
        Event::Mouse(mouse_event) => match mouse_event.kind {
            MouseEventKind::ScrollUp => {
                if chat.ui_state.layout_areas.unwrap()[1].contains(Position {
                    x: mouse_event.column,
                    y: mouse_event.row,
                }) {
                    chat.scroll_up();
                }
            }
            MouseEventKind::ScrollDown => {
                if chat.ui_state.layout_areas.unwrap()[1].contains(Position {
                    x: mouse_event.column,
                    y: mouse_event.row,
                }) {
                    chat.scroll_down();
                }
            }
            _ => {}
        },
        Event::Resize(_, _) => {
            // Set state changed; Reloads UI
            return Ok(true);
        }
        _ => {}
    }

    // No state changes; Continue as normal
    Ok(false)
}

use std::io::Result;

use crossterm::event::{self, Event, KeyCode, KeyModifiers, MouseEventKind};
use ratatui::layout::Position;

use crate::{
    app::{App, AppState, Message, MessageType},
    ui::{reset_message_box, MessageStyle},
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

            KeyCode::Char('n')
                if key.modifiers.contains(KeyModifiers::CONTROL)
                    && !chat.message_box.is_empty() =>
            {
                chat.message_box.insert_newline();
                chat.ui_state.typing_lines += 1;
                return Ok(true);
            }

            KeyCode::Enter => {
                if chat.message_box.is_empty() {
                    return Ok(false);
                }

                let mut message_text = String::new();
                for line in chat.message_box.lines() {
                    if !line.trim().is_empty() && !line.starts_with('\n') {
                        message_text.push_str((line.to_owned() + "\n").as_str());
                    }
                }

                if message_text.is_empty() {
                    return Ok(false);
                }

                if message_text.ends_with('\n') {
                    message_text.pop();
                }

                let message = Message {
                    id: "".into(),
                    channel_id: "".into(),
                    created_at: "".into(),
                    updated_at: "".into(),
                    user_id: "You".into(),
                    text: message_text,
                    message_type: MessageType::Normal,
                    lines: None,
                    style: MessageStyle::default(),
                };
                reset_message_box(chat);

                chat.send_message(message).await;

                // Bring scrolled chat back down to show sent message
                *app.chats[app.active_chat]
                    .ui_state
                    .chat_list_state
                    .offset_mut() = 0;

                // Set state changed; Reloads UI
                return Ok(true);
            }
            _ => {
                chat.message_box.input(event);
            }
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

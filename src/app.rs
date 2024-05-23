use std::io::Result;

use ratatui::{backend::Backend, Terminal};

use crate::events::handle_events;
use crate::ui::draw_ui;

pub enum CurrentScreen {
    Chat,
}

#[derive(Eq, PartialEq)]
pub enum AppState {
    Active,
    Exit,
}

pub enum ChatType {
    DM,
    // Group,
    // Server,
}
pub struct Chat {
    pub chat_type: ChatType,
    pub messages: Vec<String>,
    pub typing_message: String,
}

pub struct App {
    pub app_state: AppState,
    pub current_screen: CurrentScreen,

    pub chats: Vec<Chat>,
    pub active_chat: usize,
}

impl App {
    pub fn new() -> App {
        App {
            app_state: AppState::Active,
            current_screen: CurrentScreen::Chat,
            chats: vec![Chat {
                chat_type: ChatType::DM,
                messages: Vec::new(),
                typing_message: String::new(),
            }],
            active_chat: 0,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        while self.app_state != AppState::Exit {
            terminal.draw(|frame| draw_ui(frame, self))?;
            handle_events(self)?;
        }
        Ok(())
    }

    pub fn send_message(&mut self, message: String, chat_index: usize) {
        if chat_index >= self.chats.len() {
            return;
        };

        self.chats[chat_index].messages.push(message);
    }
}

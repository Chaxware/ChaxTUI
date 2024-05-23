use std::io::Result;

use ratatui::{backend::Backend, Terminal};

use crate::api::{Api, MessageItem};
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
    pub messages: Vec<MessageItem>,
    pub typing_message: String,
}

pub struct App {
    pub app_state: AppState,
    pub current_screen: CurrentScreen,

    pub api: Api,

    pub chats: Vec<Chat>,
    pub active_chat: usize,
}

impl App {
    pub fn new() -> App {
        App {
            app_state: AppState::Active,
            current_screen: CurrentScreen::Chat,
            api: Api::new(),
            chats: vec![Chat {
                chat_type: ChatType::DM,
                messages: Vec::new(),
                typing_message: String::new(),
            }],
            active_chat: 0,
        }
    }

    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        self.update_messages().await;
        while self.app_state != AppState::Exit {
            terminal.draw(|frame| draw_ui(frame, self))?;
            handle_events(self).await?;
        }
        Ok(())
    }

    pub async fn send_message(&mut self, message: MessageItem) {
        if self.api.send_message(&message.text).await.is_ok() {
            self.chats[self.active_chat].messages.push(message);
        }
    }

    pub async fn update_messages(&mut self) {
        self.chats[self.active_chat].messages = Vec::new();

        let fetch_result = self.api.fetch_messages().await;
        match fetch_result {
            Ok(result) => {
                self.chats[self.active_chat].messages = result.messages;
            }
            Err(_) => {
                eprintln!("Failed to fetch messages");
            }
        }
    }
}

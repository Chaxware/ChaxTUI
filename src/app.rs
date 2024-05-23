use std::io::Result;

use ratatui::{backend::Backend, Terminal};

use crate::api::Api;
use crate::events::handle_events;
use crate::ui::draw_ui;

pub enum CurrentScreen {
    Chat,
}

#[derive(PartialEq, Eq)]
pub enum AppState {
    Active,
    Exit,
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

    pub async fn send_message(&mut self, mut message: Message) {
        match self.api.send_message(&message.text).await {
            Ok(_) => {
                self.chats[self.active_chat].messages.push(message);
            }
            Err(e) => {
                message.message_type = MessageType::Unsent;
                self.chats[self.active_chat].messages.push(message);
                self.show_error(format!("Failed to send message: {}", e));
            }
        }
    }

    pub fn show_error(&mut self, error: String) {
        self.chats[self.active_chat].messages.push(Message {
            id: "".into(),
            time: "".into(),
            author: "System".into(),
            text: error,
            message_type: MessageType::SystemError,
        });
    }

    pub async fn update_messages(&mut self) {
        self.chats[self.active_chat].messages = Vec::new();

        let fetch_result = self.api.fetch_messages().await;
        match fetch_result {
            Ok(result) => {
                self.chats[self.active_chat].messages = Vec::new();
                for message in result.messages {
                    self.chats[self.active_chat].messages.push(Message {
                        id: message.id,
                        time: message.created_at,
                        author: "Someone".into(),
                        text: message.text,
                        message_type: MessageType::Normal,
                    })
                }
            }
            Err(e) => {
                self.show_error(format!("Failed to fetch messages: {}", e));
            }
        }
    }
}

pub enum ChatType {
    DM,
    // Group,
    // Server,
}

pub struct Chat {
    pub chat_type: ChatType,
    pub messages: Vec<Message>,
    pub typing_message: String,
}

pub enum MessageType {
    Normal,
    Unsent,
    SystemError,
}

pub struct Message {
    pub id: String,
    pub time: String,
    pub author: String,

    pub text: String,

    pub message_type: MessageType,
}

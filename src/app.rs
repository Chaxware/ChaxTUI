use std::io::Result;

use ratatui::{backend::Backend, Terminal};

use crate::api::Api;
use crate::events::handle_events;
use crate::ui::chat::{refresh_chat, ChatUiState};
use crate::ui::{draw_ui, load_ui, MessageStyle};

pub enum CurrentScreen {
    Chat,
}

#[derive(PartialEq, Eq)]
pub enum AppState {
    Active,
    Exit,
}

pub struct App<'a> {
    pub app_state: AppState,
    pub current_screen: CurrentScreen,

    pub chats: Vec<Chat<'a>>,
    pub active_chat: usize,
}

impl<'a> App<'a> {
    pub fn new(backend_base_url: String) -> Self {
        Self {
            app_state: AppState::Active,
            current_screen: CurrentScreen::Chat,
            chats: vec![Chat::new(ChatType::Group, backend_base_url)],
            active_chat: 0,
        }
    }

    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        self.chats[self.active_chat].update_messages().await;
        terminal.draw(|frame| {
            load_ui(frame, &mut self.chats[self.active_chat]);
            draw_ui(frame, &mut self.chats[self.active_chat]);
        })?;

        while self.app_state != AppState::Exit {
            let state_changed = handle_events(self).await?;

            terminal.draw(|frame| {
                if state_changed {
                    load_ui(frame, &mut self.chats[self.active_chat]);
                }
                draw_ui(frame, &mut self.chats[self.active_chat]);
            })?;
        }
        Ok(())
    }
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

    pub lines: Option<usize>,
    pub style: MessageStyle,
}

pub enum ChatType {
    // DM,
    Group,
    // Server,
}

pub struct Chat<'a> {
    pub api: Api,

    pub chat_type: ChatType,
    pub messages: Vec<Message>,

    pub ui_state: ChatUiState<'a>,
}

impl<'a> Chat<'a> {
    pub fn new(chat_type: ChatType, backend_base_url: String) -> Self {
        let new_chat = Self {
            api: Api::new(backend_base_url),
            chat_type,

            messages: Vec::new(),

            ui_state: ChatUiState::new(),
        };

        new_chat
    }

    pub async fn send_message(&mut self, mut message: Message) {
        match self.api.send_message(&message.text).await {
            Ok(_) => {
                self.messages.push(message);
            }
            Err(e) => {
                message.message_type = MessageType::Unsent;
                self.messages.push(message);
                self.show_error(format!("Failed to send message: {}", e));
            }
        }
        refresh_chat(self);
    }

    pub fn show_error(&mut self, error: String) {
        self.messages.push(Message {
            id: "".into(),
            time: "".into(),
            author: "System".into(),
            text: error,
            message_type: MessageType::SystemError,
            lines: None,
            style: MessageStyle::default(),
        });
    }

    pub async fn update_messages(&mut self) {
        let fetch_result = self.api.fetch_messages().await;
        match fetch_result {
            Ok(result) => {
                self.messages = Vec::new();
                for message in result.messages {
                    self.messages.push(Message {
                        id: message.id,
                        time: message.created_at,
                        author: "Someone".into(),
                        text: message.text,
                        message_type: MessageType::Normal,
                        lines: None,
                        style: MessageStyle::default(),
                    })
                }
            }
            Err(e) => {
                self.show_error(format!("Failed to fetch messages: {}", e));
            }
        }
    }
}

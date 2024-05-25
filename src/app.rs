use std::io::Result;

use ratatui::widgets::{ListItem, ListState};
use ratatui::{backend::Backend, Terminal};

use crate::api::Api;
use crate::events::handle_events;
use crate::ui::{self, draw_ui, refresh_chat};

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
        terminal.draw(|frame| draw_ui(frame, &mut self.chats[self.active_chat]))?;

        while self.app_state != AppState::Exit {
            handle_events(self).await?;
            terminal.draw(|frame| draw_ui(frame, &mut self.chats[self.active_chat]))?;
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

    pub typing_message: String,
    pub chat_list_items: Vec<ListItem<'a>>,

    pub chat_list_state: ListState,
    pub visible_messages: Option<usize>,
}

impl<'a> Chat<'a> {
    pub fn new(chat_type: ChatType, backend_base_url: String) -> Self {
        let mut new_chat = Self {
            api: Api::new(backend_base_url),
            chat_type,

            messages: Vec::new(),

            typing_message: String::new(),
            chat_list_items: Vec::new(),

            chat_list_state: ListState::default(),
            visible_messages: None,
        };

        ui::refresh_chat(&mut new_chat);

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
        });
        refresh_chat(self);
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
                    })
                }
            }
            Err(e) => {
                self.show_error(format!("Failed to fetch messages: {}", e));
            }
        }
        refresh_chat(self);
    }
}

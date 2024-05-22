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
    pub typing_message: String,
}

pub struct App {
    pub app_state: AppState,
    pub current_screen: CurrentScreen,
    pub active_chat: Chat,
}

impl App {
    pub fn new() -> App {
        App {
            app_state: AppState::Active,
            current_screen: CurrentScreen::Chat,
            active_chat: Chat {
                chat_type: ChatType::DM,
                typing_message: String::new(),
            },
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        while self.app_state != AppState::Exit {
            terminal.draw(|frame| draw_ui(frame, self))?;
            handle_events(self)?;
        }
        Ok(())
    }

    pub fn send_message(&self, message: &str) {
        println!("{}", message);
        // TODO: Implement (at least a fake) message send
    }
}

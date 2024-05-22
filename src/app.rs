use std::io::Result;

use ratatui::{backend::Backend, Terminal};

use crate::events::handle_events;
use crate::ui::draw_ui;

pub enum CurrentScreen {
    DM,
}

#[derive(Eq, PartialEq)]
pub enum AppState {
    Active,
    Exit,
}

pub struct App {
    pub app_state: AppState,
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> App {
        App {
            app_state: AppState::Active,
            current_screen: CurrentScreen::DM,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        while self.app_state != AppState::Exit {
            terminal.draw(draw_ui)?;
            handle_events(self)?;
        }
        Ok(())
    }
}

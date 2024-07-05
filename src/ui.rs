use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        Block, BorderType, Borders, List, ListDirection, Padding, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState,
    },
    Frame,
};
use tui_textarea::TextArea;

use crate::app::Chat;

use self::chat::{calculate_visible_messages, refresh_chat};

pub mod chat;

pub struct MessageStyle {
    pub text: Style,
    pub author: Style,
}

impl MessageStyle {
    pub fn default() -> Self {
        Self {
            text: Style::default(),
            author: Style::default().bold(),
        }
    }
}

// Resets the UI state
pub fn load_ui(frame: &mut Frame, chat: &mut Chat) -> Option<[Rect; 3]> {
    // Calculate window areas
    chat.ui_state.layout_areas = Some(
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(3),
                Constraint::Length((chat.ui_state.typing_lines + 4).min(frame.size().height - 6)),
            ])
            .areas(frame.size()),
    );

    // Reset message list and recalculate lines and word wrap
    refresh_chat(chat);
    calculate_visible_messages(chat);

    chat.ui_state.layout_areas
}

// Main render function
pub fn draw_ui(frame: &mut Frame, chat: &mut Chat) {
    let areas = chat
        .ui_state
        .layout_areas
        .unwrap_or_else(|| load_ui(frame, chat).unwrap());

    // Wordmark
    frame.render_widget(
        Paragraph::new("Chax")
            .centered()
            .block(Block::default().padding(Padding::new(0, 0, 1, 0))),
        areas[0],
    );

    draw_chat_window(frame, chat, areas[1]);
    draw_message_box(frame, chat, areas[2]);
}

fn draw_chat_window(frame: &mut Frame, chat: &mut Chat, area: Rect) {
    // Makes a new List from the already stored ListItem-s
    let chat_window = List::new(chat.ui_state.chat_list_items.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(2, 2, 1, 0)),
        )
        .direction(ListDirection::BottomToTop);

    // Renders according to list state (tracking offset etc.)
    frame.render_stateful_widget(chat_window, area, &mut chat.ui_state.chat_list_state);

    draw_scrollbar(frame, chat, area);
}

fn draw_scrollbar(frame: &mut Frame, chat: &mut Chat, area: Rect) {
    if chat.messages.is_empty() || chat.ui_state.visible_messages.is_none() {
        return;
    }

    let total_messages = chat.messages.len();
    let visible_messages = chat.ui_state.visible_messages.unwrap();

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .track_symbol(None)
        .begin_symbol(None)
        .end_symbol(None);
    let mut scrollbar_state = ScrollbarState::new(
        total_messages
            .saturating_sub(chat.ui_state.layout_areas.unwrap()[1].height as usize / 2 - 2),
    )
    .position(
        total_messages
            .saturating_sub(visible_messages + 1)
            .saturating_sub(chat.ui_state.chat_list_state.offset()),
    );
    frame.render_stateful_widget(
        scrollbar,
        area.inner(Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut scrollbar_state,
    );
}

pub fn reset_message_box(chat: &mut Chat) {
    chat.ui_state.typing_lines = 1;

    chat.message_box = TextArea::default();
    chat.message_box.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::new(2, 0, 1, 0)),
    );
    chat.message_box.set_placeholder_text("Write a message...");
    chat.message_box
        .set_placeholder_style(Style::default().fg(Color::DarkGray));
    chat.message_box.set_cursor_line_style(Style::default());
}

fn draw_message_box(frame: &mut Frame, chat: &mut Chat, area: Rect) {
    frame.render_widget(chat.message_box.widget(), area);
}

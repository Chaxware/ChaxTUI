use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, List, ListDirection, ListItem, ListState, Padding, Paragraph,
        Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    Frame,
};

use crate::app::{Chat, MessageType};

pub struct UiState<'a> {
    pub layout_areas: Option<[Rect; 3]>,
    pub typing_message: String,
    pub visible_messages: Option<usize>,
    pub chat_list_items: Vec<ListItem<'a>>,
    pub chat_list_state: ListState,
}

impl<'a> UiState<'a> {
    pub fn new() -> Self {
        Self {
            layout_areas: None,
            typing_message: String::new(),
            visible_messages: None,
            chat_list_items: Vec::new(),
            chat_list_state: ListState::default(),
        }
    }
}

pub fn draw_ui(frame: &mut Frame, chat: &mut Chat, state_changed: bool) {
    if state_changed {
        chat.ui_state.layout_areas = Some(
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(2),
                    Constraint::Min(3),
                    Constraint::Length(5),
                ])
                .areas(frame.size()),
        );
        chat.ui_state.visible_messages =
            Some(chat.ui_state.layout_areas.unwrap()[1].height as usize / 2 - 2);
        refresh_chat(chat);
    }

    if chat.ui_state.layout_areas.is_none() {
        return;
    }

    // Wordmark
    frame.render_widget(
        Paragraph::new("Chax")
            .centered()
            .block(Block::default().padding(Padding::new(0, 0, 1, 0))),
        chat.ui_state.layout_areas.unwrap()[0],
    );

    draw_chat_window(frame, chat, chat.ui_state.layout_areas.unwrap()[1]);
    draw_message_box(frame, chat, chat.ui_state.layout_areas.unwrap()[2]);
}

pub fn refresh_chat(chat: &mut Chat) {
    chat.ui_state.chat_list_items = Vec::new();
    for message in &chat.messages {
        let mut author_span = Span::raw(format!(" {} ", &message.author)).bold();
        let mut message_span = Span::from(message.text.clone());

        match message.message_type {
            MessageType::Normal => {
                author_span = author_span.fg(Color::Cyan);
            }
            MessageType::Unsent => {
                message_span = message_span.fg(Color::DarkGray);
            }
            MessageType::SystemError => {
                author_span = author_span.bg(Color::Red).fg(Color::DarkGray);
                message_span = message_span.fg(Color::Red);
            }
        }

        chat.ui_state
            .chat_list_items
            .push(ListItem::new(Text::from(vec![
                Line::from(vec![author_span, Span::raw(": "), message_span]),
                Line::default(),
            ])));
    }
    chat.ui_state.chat_list_items.reverse();
}

fn draw_chat_window(frame: &mut Frame, chat: &mut Chat, area: Rect) {
    let chat_window = List::new(chat.ui_state.chat_list_items.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(2, 2, 1, 0)),
        )
        .direction(ListDirection::BottomToTop);
    frame.render_stateful_widget(chat_window, area, &mut chat.ui_state.chat_list_state);

    draw_scrollbar(frame, chat, area);
}

fn draw_scrollbar(frame: &mut Frame, chat: &mut Chat, area: Rect) {
    let total_messages = chat.messages.len();
    let visible_messages = chat.ui_state.visible_messages.unwrap();

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .track_symbol(None)
        .begin_symbol(None)
        .end_symbol(None);
    let mut scrollbar_state = ScrollbarState::new(total_messages.saturating_sub(visible_messages))
        .position(
            total_messages
                .saturating_sub(visible_messages + 1)
                .saturating_sub(chat.ui_state.chat_list_state.offset()),
        );
    frame.render_stateful_widget(
        scrollbar,
        area.inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut scrollbar_state,
    );
}

fn draw_message_box(frame: &mut Frame, chat: &mut Chat, area: Rect) {
    let mut typing_message = chat.ui_state.typing_message.clone();
    let mut empty_message = false;
    if typing_message.is_empty() {
        typing_message = String::from("Write a message...");
        empty_message = true;
    }

    let message_box = Paragraph::new(Line::from(typing_message).style(if empty_message {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default()
    }))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::new(2, 0, 1, 0)),
    );
    frame.render_widget(message_box, area);
}

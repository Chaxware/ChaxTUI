use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, List, ListDirection, ListItem, Padding, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState,
    },
    Frame,
};

use crate::app::{Chat, MessageType};

pub fn draw_ui(frame: &mut Frame, chat: &mut Chat) {
    // Define layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(3),
            Constraint::Length(5),
        ])
        .split(frame.size());

    // Wordmark
    frame.render_widget(
        Paragraph::new("Chax")
            .centered()
            .block(Block::default().padding(Padding::new(0, 0, 1, 0))),
        chunks[0],
    );

    draw_chat_window(frame, chat, chunks[1]);
    draw_message_box(frame, chat, chunks[2]);
}

pub fn refresh_chat(chat: &mut Chat) {
    chat.chat_list_items = Vec::new();
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

        chat.chat_list_items.push(ListItem::from(vec![
            Line::from(vec![author_span, Span::raw(": "), message_span]),
            Line::default(),
        ]));
    }
    chat.chat_list_items.reverse();
}

fn draw_chat_window(frame: &mut Frame, chat: &mut Chat, area: Rect) {
    if chat.visible_messages.is_none() {
        chat.visible_messages = Some(area.height as usize / 2 - 2);
    }

    let chat_window = List::new(chat.chat_list_items.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(2, 2, 1, 0)),
        )
        .direction(ListDirection::BottomToTop)
        .highlight_style(Style::default().bg(Color::Green));
    frame.render_stateful_widget(chat_window, area, &mut chat.chat_list_state);

    draw_scrollbar(frame, chat, area);
}

fn draw_scrollbar(frame: &mut Frame, chat: &mut Chat, area: Rect) {
    let total_messages = chat.messages.len();
    let visible_messages = chat.visible_messages.unwrap();

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .track_symbol(None)
        .begin_symbol(None)
        .end_symbol(None);
    let mut scrollbar_state = ScrollbarState::new(total_messages.saturating_sub(visible_messages))
        .position(
            total_messages
                .saturating_sub(visible_messages + 1)
                .saturating_sub(chat.chat_list_state.offset()),
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
    let mut typing_message = chat.typing_message.clone();
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

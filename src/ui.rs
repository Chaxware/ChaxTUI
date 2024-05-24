use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListDirection, ListItem, Padding, Paragraph},
    Frame,
};

use crate::app::{App, MessageType};

pub fn draw_ui(frame: &mut Frame, app: &mut App) {
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
    let wordmark = Paragraph::new("Chax")
        .centered()
        .block(Block::default().padding(Padding::new(0, 0, 1, 0)));
    frame.render_widget(wordmark, chunks[0]);

    // Chat Window
    let mut message_list = Vec::new();
    for message in &app.chats[app.active_chat].messages {
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

        message_list.push(ListItem::from(vec![
            Line::from(vec![author_span, Span::raw(": "), message_span]),
            Line::default(),
        ]));
    }
    message_list.reverse();

    if app.chats[app.active_chat].visible_messages.is_none() {
        app.chats[app.active_chat].visible_messages = Some(chunks[1].height as usize / 2 - 2);
    }

    let chat_window = List::new(message_list)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(2, 2, 1, 0)),
        )
        .direction(ListDirection::BottomToTop)
        .highlight_style(Style::default().bg(Color::Green));
    frame.render_stateful_widget(
        chat_window,
        chunks[1],
        &mut app.chats[app.active_chat].chat_list_state,
    );

    // Message Box
    let active_chat = &app.chats[app.active_chat];
    let mut typing_message = active_chat.typing_message.clone();
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
    frame.render_widget(message_box, chunks[2]);
}

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListDirection, ListItem, Padding, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw_ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(3),
            Constraint::Length(5),
        ])
        .split(frame.size());

    let hero = Paragraph::new("Chax")
        .centered()
        .block(Block::default().padding(Padding::new(0, 0, 1, 0)));
    frame.render_widget(hero, chunks[0]);

    let mut message_list = Vec::new();
    for message in &app.chats[app.active_chat].messages {
        message_list.push(ListItem::from(vec![
            Line::from(vec![
                Span::raw("You").bold().fg(Color::Cyan),
                Span::raw(": "),
                Span::from(message.text.clone()),
            ]),
            Line::default(),
        ]));
    }
    message_list.reverse();

    let chat_window = List::new(message_list)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(2, 2, 1, 0)),
        )
        .direction(ListDirection::BottomToTop);
    frame.render_widget(chat_window, chunks[1]);

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

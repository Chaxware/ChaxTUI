use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
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

    let chat_window = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    frame.render_widget(chat_window, chunks[1]);

    let mut typing_message = app.active_chat.typing_message.clone();
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

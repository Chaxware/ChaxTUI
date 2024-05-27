use ratatui::{
    layout::Rect,
    style::Color,
    text::{Line, Span, Text},
    widgets::{ListItem, ListState},
};

use crate::app::{Chat, Message, MessageType};

pub struct ChatUiState<'a> {
    pub layout_areas: Option<[Rect; 3]>,
    pub typing_message: String,
    pub visible_messages: Option<usize>,
    pub chat_list_items: Vec<ListItem<'a>>,
    pub chat_list_state: ListState,
}

impl<'a> ChatUiState<'a> {
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

pub fn calculate_visible_messages(chat: &mut Chat) {
    if chat.messages.is_empty() {
        return;
    }

    let mut current_message = 0;
    let mut available_height = chat.ui_state.layout_areas.unwrap()[1].height as usize - 4;
    let mut visible_messages: usize = 0;

    loop {
        let current_message_lines = chat.messages[current_message].lines.unwrap();
        available_height = available_height.saturating_sub(current_message_lines);
        visible_messages += 1;
        current_message += 1;

        if current_message == chat.messages.len() || available_height == 0 {
            break;
        }
    }

    chat.ui_state.visible_messages = Some(
        visible_messages
            .saturating_sub(1)
            .min(chat.ui_state.layout_areas.unwrap()[1].height as usize / 2 - 2),
    );
}

pub fn wrap_message(message: &Message, max_width: usize) -> Vec<String> {
    let mut strings = Vec::new();

    let pre_text_width = message.author.len() + 4;
    let mut scanner_start_position = 0;
    let mut message_length_left = message.text.len();

    let mut first_line = true;

    while message_length_left > 0 {
        let available_width = if first_line {
            max_width.saturating_sub(pre_text_width)
        } else {
            max_width
        };

        if available_width == 0 {
            if first_line {
                strings.push(String::new());
            } else {
                break;
            }
        }

        first_line = false;

        if message_length_left <= available_width {
            strings.push(message.text[scanner_start_position..].to_string());
            break;
        }

        let mut scanner_end_position = scanner_start_position + available_width;
        if scanner_end_position >= message.text.len() {
            scanner_end_position = message.text.len();
        }

        while scanner_end_position > scanner_start_position {
            let c = message.text.as_bytes()[scanner_end_position - 1];
            if c.is_ascii_whitespace() {
                break;
            }
            scanner_end_position -= 1;
        }

        if scanner_end_position == scanner_start_position {
            scanner_end_position = scanner_start_position + available_width;
            if scanner_end_position > message.text.len() {
                scanner_end_position = message.text.len();
            }
        }

        strings.push(message.text[scanner_start_position..scanner_end_position].to_string());

        scanner_start_position = scanner_end_position;
        while scanner_start_position < message.text.len()
            && message.text.as_bytes()[scanner_start_position].is_ascii_whitespace()
        {
            scanner_start_position += 1;
        }

        message_length_left = message.text.len().saturating_sub(scanner_start_position);
    }

    strings
}

pub fn refresh_chat(chat: &mut Chat) {
    let max_width = (chat.ui_state.layout_areas.unwrap()[1].width as usize).saturating_sub(6);

    chat.ui_state.chat_list_items.clear();
    for message in &mut chat.messages {
        match message.message_type {
            MessageType::Normal => {
                message.style.author = message.style.author.fg(Color::Cyan);
            }
            MessageType::Unsent => {
                message.style.text = message.style.text.fg(Color::DarkGray);
            }
            MessageType::SystemError => {
                message.style.author = message.style.author.bg(Color::Red).fg(Color::DarkGray);
                message.style.text = message.style.text.fg(Color::Red);
            }
        }

        let author_span = Span::from(format!(" {} ", &message.author)).style(message.style.author);

        let mut lines: Vec<Line> = Vec::new();
        if message.author.len() + message.text.len() + 4 <= max_width {
            lines.push(Line::from(vec![
                author_span,
                Span::from(": "),
                Span::from(message.text.clone()).style(message.style.text),
            ]));
        } else {
            let line_strings = wrap_message(message, max_width);
            if !line_strings.is_empty() {
                lines.push(Line::from(vec![
                    author_span,
                    Span::from(": "),
                    Span::from(line_strings[0].clone()).style(message.style.text),
                ]));
                for line_string in line_strings.iter().skip(1) {
                    lines.push(Line::from(line_string.clone()).style(message.style.text));
                }
            }
        }

        lines.push(Line::default());

        message.lines = Some(lines.len());

        chat.ui_state
            .chat_list_items
            .push(ListItem::new(Text::from(lines)));
    }
    chat.ui_state.chat_list_items.reverse();
}

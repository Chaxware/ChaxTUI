use ratatui::{
    layout::Rect,
    style::Color,
    text::{Line, Span, Text},
    widgets::{ListItem, ListState},
};

use crate::app::{Chat, Message, MessageType};

pub struct ChatUiState<'a> {
    // The areas of the windows
    pub layout_areas: Option<[Rect; 3]>,

    // The currently typing message
    pub typing_message: String,

    // The no. of visible messages at the top
    pub visible_messages: Option<usize>,

    // Cache for the chat list items (i.e. messages)
    pub chat_list_items: Vec<ListItem<'a>>,

    // The state of the list (offset etc.)
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

// Calculates the no. of visible messages at the top, so that you can
// actually see all the messages even with different no. of lines on
// each message, and get an (almost) accurate scrollbar
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

    // It has a maximum limit of about half the height of the window,
    // as each message takes up at least 2 lines
    //
    // And the -1 is because we are offsetting from bottom, and
    // without it, the top-most message might be inaccessible
    chat.ui_state.visible_messages = Some(
        visible_messages
            .saturating_sub(1)
            .min(chat.ui_state.layout_areas.unwrap()[1].height as usize / 2 - 2),
    );
}

// Break a single line message into multiple lines if it doesn't fit
// on the current window width
pub fn wrap_message(message: &Message, max_width: usize) -> Vec<String> {
    let mut strings = Vec::new();

    // For the author's name
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
                // If the author's name takes up too much space
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

        // Check for whitespace to break off the line with
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

        // Otherwise just break whatever the width cuts on (might be mid-word)
        strings.push(message.text[scanner_start_position..scanner_end_position].to_string());
        scanner_start_position = scanner_end_position;

        // Check for chain of whitespaces
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

    // For each message in the local store, make a ListItem
    for message in &mut chat.messages {
        // Message and Author name styling
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

        // If everything fits snugly, just push it in easily
        if message.author.len() + message.text.len() + 4 <= max_width {
            lines.push(Line::from(vec![
                author_span,
                Span::from(": "),
                Span::from(message.text.clone()).style(message.style.text),
            ]));
        }
        // Otherwise do some wrapping up
        else {
            // Get the multi-lined strings
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

        // Extra line for padding
        lines.push(Line::default());

        message.lines = Some(lines.len()); // Update UI state

        chat.ui_state
            .chat_list_items
            .push(ListItem::new(Text::from(lines)));
    }

    // Reverse the list, because we are doing a BottomToTop list
    // So, the bottom-most message is index 0, and so on
    chat.ui_state.chat_list_items.reverse();
}

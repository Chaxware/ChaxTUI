use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    text::Line,
    widgets::{block::Title, Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

pub fn draw_ui(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(frame.size());
    let sub_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(chunks[1]);

    let greeting_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Title::from(" Hello World ").alignment(Alignment::Center))
        .padding(Padding::new(0, 0, sub_chunks[1].height / 2 - 1, 0));
    let greeting = Paragraph::new(Line::from("Greetings, Stranger!"))
        .centered()
        .block(greeting_block);
    frame.render_widget(greeting, sub_chunks[1]);
}

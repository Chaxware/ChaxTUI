use std::io::{stdout, Result};

use crossterm::{
    event, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::{block::Title, Block, BorderType, Borders, Padding, Paragraph},
    Terminal,
};

fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.draw(|frame| {
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
            .bold()
            .block(greeting_block);
        frame.render_widget(greeting, sub_chunks[1]);
    })?;

    event::read()?;

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}

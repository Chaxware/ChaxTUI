use std::io::{stdout, Result};

use app::App;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod api;
mod app;
mod events;
mod ui;

#[derive(Parser)]
struct Arguments {
    backend_base_url: String,
    hub_id: String,
    channel_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arguments::parse();
    let backend_base_url = args.backend_base_url;
    let hub_id = args.hub_id;
    let channel_id = args.channel_id;

    let channel_url: String = format!("{}/chat/{}/{}", backend_base_url, hub_id, channel_id);

    // Enter TUI screen
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), EnableMouseCapture)?;
    execute!(stdout(), Clear(ClearType::Purge))?;
    execute!(stdout(), Clear(ClearType::All))?;

    // Initialize terminal backend
    let terminal_backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(terminal_backend)?;

    // Initialize and run app
    let mut app = App::new(channel_url);
    app.run(&mut terminal).await?;

    // Exit TUI screen
    disable_raw_mode()?;
    execute!(stdout(), DisableMouseCapture)?;
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}

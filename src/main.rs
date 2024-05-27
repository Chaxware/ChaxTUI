use std::{
    env,
    io::{stdout, Result},
};

use app::App;
use crossterm::{
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

#[tokio::main]
async fn main() -> Result<()> {
    let mut backend_base_url: String = env::args()
        .nth(1)
        .unwrap_or_else(|| "http://localhost:8787".into());

    if backend_base_url.ends_with('/') {
        backend_base_url.pop();
    }

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), Clear(ClearType::Purge))?;
    execute!(stdout(), Clear(ClearType::All))?;

    let terminal_backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(terminal_backend)?;

    let mut app = App::new(backend_base_url);
    app.run(&mut terminal).await?;

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}

use crate::app::{AppState, run};
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io::stdout;
mod app;
pub mod downloading;
pub mod getting_location;
pub mod getting_weather;
mod ui;

#[tokio::main]
async fn main() {
    ratatui::init();
    let mut app: AppState;
    if let Ok(app_state) = AppState::new() {
        app = app_state
    } else {
        todo!("Error retrieving app state");
    }
    let backend = CrosstermBackend::new(stdout());
    let mut terminal;
    if let Ok(terminal_sec) = Terminal::new(backend) {
        terminal = terminal_sec
    } else {
        todo!("Error retrieving terminal");
    }
    let _ = run(&mut terminal, &mut app).await.unwrap();
    ratatui::restore()
}

use crate::app::{AppState, run};
mod app;
pub mod downloading;
pub mod getting_location;
pub mod getting_weather;
mod ui;

#[tokio::main]
async fn main() {
    let mut terminal = ratatui::init();
    let mut app: AppState;
    if let Ok(app_state) = AppState::new() {
        app = app_state
    } else {
        todo!("Error retrieving app state");
    }
    let _ = run(&mut terminal, &mut app).await.unwrap();
    ratatui::restore()
}

use crate::app::{AppState, run};
use crate::error::error;
mod app;
mod downloading;
mod error;
mod getting_location;
mod getting_weather;
mod ui;
use smol_macros::main;
main! {
async fn main() {
        let mut terminal = ratatui::init();
        let mut app: AppState;
        if let Ok(app_state) = AppState::new() {
            app = app_state;
            if let Err(e) = run(&mut terminal, &mut app).await{
                error(&mut terminal, &e);
            }
        }
        if let Err(e) = AppState::new() {
                error(&mut terminal, &e);
        }
        ratatui::restore()
    }
}
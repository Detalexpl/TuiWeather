use ratatui::Frame;
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::layout::{Constraint, Layout};
use crate::app::AppState;

pub fn ui(frame: &mut Frame, app: &mut AppState){
    let chunks = Layout::default()
        .direction(Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());
    
}
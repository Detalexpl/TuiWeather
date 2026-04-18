use ratatui::layout::Direction::Vertical;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Backend, Color};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders};
use ratatui::{Frame, Terminal};

pub fn error<B: Backend>(terminal: &mut Terminal<B>, mg: &str) {
    loop {
        terminal.draw(|f| draw_error(f, mg));
        if crossterm::event::read().unwrap().is_key_press() {
            break;
        }
    }
    ratatui::restore();
    std::process::exit(1);
}
fn draw_error(frame: &mut Frame, mg: &str) {
    let layout = Layout::default()
        .direction(Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(frame.area());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(
            Style::default()
                .bg(Color::Rgb(0, 0, 0))
                .fg(Color::Rgb(255, 0, 0)),
        );
    let msg = format!("Program run into problem: {}", mg);
    let msg = Line::from(msg).centered();
    frame.render_widget(block, frame.area());
    frame.render_widget(msg, layout[1]);
}

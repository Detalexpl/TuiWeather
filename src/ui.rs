use crossterm::style::Color::DarkGreen;
use ratatui::Frame;
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, TitlePosition};
use crate::app::AppState;
struct ColorPalette{
    bg:Color,
    fg:Color,
}
impl ColorPalette {
    pub fn get_colors(app: &AppState) -> ColorPalette {
        if let Some(weather)= &app.weather {
            if weather.weather_code == 0{
                return ColorPalette{bg:Color::Blue, fg:Color::Yellow, };
            }
            else if weather.weather_code == 1 || weather.weather_code == 2 || weather.weather_code == 3 {
                return ColorPalette{bg:Color::LightBlue ,fg:Color::Yellow};
            }else if weather.weather_code == 45 || weather.weather_code == 48 {
                return ColorPalette{bg:Color::Gray,fg:Color::Black};
            }
            else if weather.weather_code == 56 || weather.weather_code == 57 {
                return ColorPalette{bg:Color::Gray,fg:Color::LightBlue};
            }else if weather.weather_code == 61 || weather.weather_code == 63 || weather.weather_code == 65 {
                return ColorPalette{bg:Color::Gray,fg:Color::LightBlue};
            }else if weather.weather_code == 66 || weather.weather_code == 67 {
                return ColorPalette{bg:Color::DarkGray,fg:Color::Black};
            }else if weather.weather_code == 71 || weather.weather_code == 73 || weather.weather_code == 75 {
                return ColorPalette{bg:Color::Gray,fg:Color::LightCyan};
            }else if weather.weather_code == 77{
                return ColorPalette{bg:Color::DarkGray,fg:Color::White};
            }else if weather.weather_code == 80 || weather.weather_code == 81 || weather.weather_code == 82 {
                return ColorPalette{bg:Color::Gray,fg:Color::Blue};
            }else if weather.weather_code == 85 || weather.weather_code == 86 {
                return ColorPalette{bg:Color::DarkGray,fg:Color::White};
            }else if weather.weather_code == 95 || weather.weather_code == 96 || weather.weather_code == 99 {
                return ColorPalette{bg:Color::DarkGray,fg:Color::LightYellow};
            }
            else{
                return ColorPalette{bg:Color::default(),fg:Color::default()};
            }
        }else{
            return ColorPalette{bg:Color::default(),fg:Color::default()};
        }
    }
}

pub fn ui(frame: &mut Frame, app: &mut AppState){

    let chunks = Layout::default()
        .direction(Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(ColorPalette::get_colors(&app).bg))
        .border_type(BorderType::HeavyTripleDashed)
        .title("Location")
        .title_alignment(Alignment::Center);
    let title = Paragraph::new(Text::from(app.location.clone()))
        .style(Style::default()
            //.fg(ColorPalette::get_colors(&app).fg))
        .fg(Color::Blue))
        .block(title_block);

    frame.render_widget(title, chunks[0]);

}

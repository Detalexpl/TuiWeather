use crate::app::{AppState, Mode};
use crossterm::event::KeyCode::F;
use crossterm::style::Color::DarkGreen;
use ratatui::Frame;
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, TitlePosition};


struct ColorPalette {
    bg: Color,
    fg: Color,
}
impl ColorPalette {
    pub fn get_colors(app: &AppState) -> ColorPalette {
        if let Some(weather) = &app.weather {
            if weather.weather_code == 0 {
                return ColorPalette {
                    bg: Color::Blue,
                    fg: Color::Yellow,
                };
            } else if weather.weather_code == 1
                || weather.weather_code == 2
                || weather.weather_code == 3
            {
                return ColorPalette {
                    bg: Color::LightBlue,
                    fg: Color::Yellow,
                };
            } else if weather.weather_code == 45 || weather.weather_code == 48 {
                return ColorPalette {
                    bg: Color::Gray,
                    fg: Color::Black,
                };
            } else if weather.weather_code == 56 || weather.weather_code == 57 {
                return ColorPalette {
                    bg: Color::Gray,
                    fg: Color::LightBlue,
                };
            } else if weather.weather_code == 61
                || weather.weather_code == 63
                || weather.weather_code == 65
            {
                return ColorPalette {
                    bg: Color::Gray,
                    fg: Color::LightBlue,
                };
            } else if weather.weather_code == 66 || weather.weather_code == 67 {
                return ColorPalette {
                    bg: Color::DarkGray,
                    fg: Color::Black,
                };
            } else if weather.weather_code == 71
                || weather.weather_code == 73
                || weather.weather_code == 75
            {
                return ColorPalette {
                    bg: Color::Gray,
                    fg: Color::LightCyan,
                };
            } else if weather.weather_code == 77 {
                return ColorPalette {
                    bg: Color::DarkGray,
                    fg: Color::White,
                };
            } else if weather.weather_code == 80
                || weather.weather_code == 81
                || weather.weather_code == 82
            {
                return ColorPalette {
                    bg: Color::Gray,
                    fg: Color::Blue,
                };
            } else if weather.weather_code == 85 || weather.weather_code == 86 {
                return ColorPalette {
                    bg: Color::DarkGray,
                    fg: Color::White,
                };
            } else if weather.weather_code == 95
                || weather.weather_code == 96
                || weather.weather_code == 99
            {
                return ColorPalette {
                    bg: Color::DarkGray,
                    fg: Color::LightYellow,
                };
            } else {
                return ColorPalette {
                    bg: Color::default(),
                    fg: Color::default(),
                };
            }
        } else {
            return ColorPalette {
                bg: Color::default(),
                fg: Color::default(),
            };
        }
    }
}

pub fn ui(frame: &mut Frame, app: &mut AppState) {
    let colors = ColorPalette::get_colors(&app);
    //this part of code is used to reate Layout
    let chunks = Layout::default()
        .direction(Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());
    let header_chunks = Layout::default()
        .direction(Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Min(15)])
        .split(chunks[0]);
    let main_chunks = Layout::default()
        .direction(Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(chunks[1]);
    let footer_chunks = Layout::default()
        .direction(Horizontal)
        .constraints([
            Constraint::Length(15),
            Constraint::Min(15),
        ])
        .split(chunks[2]);
    let location_block = Block::default()
        .borders(Borders::ALL)
        .title("Location")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(colors.bg))
        .border_type(BorderType::Rounded);

    let location = Paragraph::new(Text::from(app.location.clone()))
        .style(Style::default().fg(colors.fg))
        .block(location_block);
    let battery_block = Block::default()
        .borders(Borders::ALL)
        .title("Battery")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(colors.bg).fg(colors.fg))
        .border_type(BorderType::Rounded);
    //    let battery = Paragraph::new(Text::from(""))
    //        .style(Style::default().fg(colors.fg))
    //        .block(battery_block);
    let mut batteries = Vec::<ListItem>::new();

    for percentage in &app.battery {
        if percentage >= &40.0 {
            batteries.push(ListItem::new(Line::from(Span::styled(
                format!("Battery: {:.0}%", percentage),
                Style::default().fg(Color::Green),
            ))));
        } else if percentage > &15.0 {
            batteries.push(ListItem::new(Line::from(Span::styled(
                format!("Battery: {:.0}%", percentage),
                Style::default().fg(Color::Yellow),
            ))));
        } else {
            batteries.push(ListItem::new(Line::from(Span::styled(
                format!("Battery: {:.0}%", percentage),
                Style::default().fg(Color::Red),
            ))))
        }
    }
    let list = List::new(batteries).block(battery_block);
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Weather")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(colors.bg).fg(colors.fg));
    let cheat_sheet_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("hints")
        .title_alignment(Alignment::Center);
    let mut temp = String::from("");
    if let Some( weather) = app.weather.clone() {
        temp = weather.temperature_2m.to_string()
    }
    let main = Paragraph::new(
        Text::from(Span::styled(format!("temp: {}", temp), Style::default().fg(colors.fg))),
    )
        .centered()
        .style(Style::default().fg(colors.fg))
        .block(main_block);
    frame.render_widget(main, main_chunks[0]);
    frame.render_widget(list, header_chunks[1]);
    frame.render_widget(location, header_chunks[0]);
    match app.mode{
        Mode::Typing => {
            let typing_chunk = centered_rect(50,50,frame.area());
            let typing_block = Block::default().borders(Borders::ALL).title("Typing").title_alignment(Alignment::Center).style(Style::default().bg(Color::Black));
            let typing= Paragraph::new(Line::from(format!("insert your location: {}", app.location_input ))).block(typing_block);
            frame.render_widget(Clear, typing_chunk);
            frame.render_widget(typing, typing_chunk);
        }
        _ => {}
    }


    //frame.render_widget(battery, header_chunks[1]);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
// Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

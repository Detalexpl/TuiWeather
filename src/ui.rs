use crate::app::{AppState, Mode};
use ratatui::Frame;
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Tabs};

struct ColorPalette {
    bg: Color,
    fg: Color,
}
impl ColorPalette {
    pub fn get_colors(app: &AppState) -> ColorPalette {
        if let Some(weather) = &app.weather {
            if weather.weather_code == 0 {
                ColorPalette {
                    bg: Color::Blue,
                    fg: Color::Yellow,
                }
            } else if weather.weather_code == 1
                || weather.weather_code == 2
                || weather.weather_code == 3
            {
                ColorPalette {
                    bg: Color::LightBlue,
                    fg: Color::Yellow,
                }
            } else if weather.weather_code == 45 || weather.weather_code == 48 {
                ColorPalette {
                    bg: Color::Gray,
                    fg: Color::Black,
                }
            } else if weather.weather_code == 56 || weather.weather_code == 57 {
                ColorPalette {
                    bg: Color::Gray,
                    fg: Color::LightBlue,
                }
            } else if weather.weather_code == 61
                || weather.weather_code == 63
                || weather.weather_code == 65
            {
                ColorPalette {
                    bg: Color::Gray,
                    fg: Color::LightBlue,
                }
            } else if weather.weather_code == 66 || weather.weather_code == 67 {
                ColorPalette {
                    bg: Color::DarkGray,
                    fg: Color::Black,
                }
            } else if weather.weather_code == 71
                || weather.weather_code == 73
                || weather.weather_code == 75
            {
                ColorPalette {
                    bg: Color::Gray,
                    fg: Color::LightCyan,
                }
            } else if weather.weather_code == 77 {
                ColorPalette {
                    bg: Color::DarkGray,
                    fg: Color::White,
                }
            } else if weather.weather_code == 80
                || weather.weather_code == 81
                || weather.weather_code == 82
            {
                ColorPalette {
                    bg: Color::Gray,
                    fg: Color::Blue,
                }
            } else if weather.weather_code == 85 || weather.weather_code == 86 {
                ColorPalette {
                    bg: Color::DarkGray,
                    fg: Color::White,
                }
            } else if weather.weather_code == 95
                || weather.weather_code == 96
                || weather.weather_code == 99
            {
                ColorPalette {
                    bg: Color::DarkGray,
                    fg: Color::LightYellow,
                }
            } else {
                ColorPalette {
                    bg: Color::default(),
                    fg: Color::default(),
                }
            }
        } else {
            ColorPalette {
                bg: Color::default(),
                fg: Color::default(),
            }
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
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[1]);
    let main_vertical_chunks = Layout::default()
        .direction(Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);
    let footer_chunks = Layout::default()
        .direction(Horizontal)
        .constraints([
            Constraint::Min(10),
            Constraint::Percentage(60),
            Constraint::Length(5),
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
        .block(location_block)
        .centered();
    let battery_block = Block::default()
        .borders(Borders::ALL)
        .title("Battery")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(colors.bg).fg(colors.fg))
        .border_type(BorderType::Rounded);
    let mut batteries = Vec::<ListItem>::new();
    if app.battery.is_empty() {
        frame.render_widget(location, chunks[0])
    } else {
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

        frame.render_widget(list, header_chunks[1]);
        frame.render_widget(location, header_chunks[0]);
    }
    // most of the blocks shode be there
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
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(colors.bg).fg(colors.fg));
    let last_char_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(colors.fg).bg(colors.bg));
    let time_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Time")
        .title_alignment(Alignment::Center)
        .bg(colors.bg)
        .fg(colors.fg);
    let mut temp = String::from("");
    let wind_direction_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Wind Direction")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(colors.bg).fg(colors.fg));
    let wind_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Wind")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(colors.bg).fg(colors.fg));
    if let Some(weather) = app.weather.clone() {
        temp = weather.temperature_2m.to_string()
    }
    let main = Paragraph::new(Text::from(Span::styled(
        format!("temp: {}", temp),
        Style::default().fg(colors.fg),
    )))
    .centered()
    .style(Style::default().fg(colors.fg))
    .block(main_block)
    .centered();
    let time = Paragraph::new(Line::from(vec![
        app.real_time
            .format("%d/%m/%y  %H:%M")
            .to_string()
            .fg(colors.fg),
    ]))
    .block(time_block)
    .centered();
    let last_char = Paragraph::new(Text::from(
        Span::styled(app.last_char.to_string(), Style::default().fg(colors.fg))
            .into_centered_line(),
    ))
    .block(last_char_block);
    let mut wind_spd = String::new();
    let mut wind_dir = String::new();
    if let Some(weather) = app.weather.clone() {
        wind_dir = weather.wind_direction_10m.to_string();
        wind_spd = weather.wind_speed_10m.to_string();
    }
    let wind_direction = Paragraph::new(Text::from(format!("wind direction: {}", wind_dir)))
        .style(Style::default().fg(colors.fg))
        .block(wind_direction_block);
    let wind_spd = Paragraph::new(Text::from(format!("wind speed: {}", wind_spd)))
        .style(Style::default().fg(colors.fg))
        .block(wind_block);
    // help info here
    match app.mode {
        Mode::Normal => {
            let cheat_sheet = Paragraph::new(
                Line::from(vec![
                    "Reload ".fg(colors.fg),
                    "<R>".fg(Color::Rgb(155, 0, 255)),
                    "   ".into(),
                    "Search location ".fg(colors.fg),
                    "<S>".fg(Color::Rgb(155, 0, 255)),
                    "   ".into(),
                    "Quit ".fg(colors.fg),
                    "<Q>".fg(Color::Rgb(255, 50, 50)),
                ])
                .centered(),
            )
            .block(cheat_sheet_block);
            frame.render_widget(cheat_sheet, footer_chunks[1]);
        }
        Mode::Typing => {
            let cheat_sheet = Paragraph::new(
                Line::from(vec![
                    "Escape ".fg(colors.fg),
                    "<ESC>".fg(Color::Rgb(155, 0, 255)),
                    "   ".into(),
                    "Delete ".fg(colors.fg),
                    "<BACKSPACE>".fg(Color::Rgb(155, 0, 255)),
                    "   ".into(),
                    "Search ".fg(colors.fg),
                    "<ENTER>".fg(Color::Rgb(155, 0, 255)),
                ])
                .centered(),
            )
            .block(cheat_sheet_block);
            frame.render_widget(cheat_sheet, footer_chunks[1]);
        }
        Mode::Exiting => {
            let cheat_sheet = Paragraph::new(Line::from(Span::styled(
                "Follow instructions on popup",
                Style::default().fg(colors.fg).bg(colors.bg),
            )))
            .block(cheat_sheet_block);
            frame.render_widget(cheat_sheet, footer_chunks[1]);
        }
        Mode::Settings => {}
    }
    // some rendering
    frame.render_widget(main, main_chunks[0]);
    frame.render_widget(wind_spd, main_vertical_chunks[0]);
    frame.render_widget(wind_direction, main_vertical_chunks[1]);
    frame.render_widget(last_char, footer_chunks[2]);
    frame.render_widget(time, footer_chunks[0]);
    // popups go here
    match app.mode {
        Mode::Typing => {
            let typing_chunk = centered_rect(35, 35, frame.area());
            let typing_block = Block::default()
                .borders(Borders::ALL)
                .title("Typing")
                .title_alignment(Alignment::Center)
                .style(Style::default().bg(Color::Black));
            let typing = Paragraph::new(Line::from(format!(
                "insert your location: {}",
                app.location_input
            )))
            .block(typing_block);
            frame.render_widget(Clear, typing_chunk);
            frame.render_widget(typing, typing_chunk);
        }
        Mode::Exiting => {
            let exiting_chunk = centered_rect(30, 30, frame.area());
            let exiting_block = Block::default()
                .borders(Borders::ALL)
                .title("Exiting")
                .title_alignment(Alignment::Center)
                .style(Style::default().bg(Color::Black));
            let exiting = Paragraph::new(Line::from(vec![
                "Do you wont to exit? ".fg(Color::Rgb(200, 100, 255)),
                "(".fg(Color::Rgb(200, 100, 255)),
                "Y".fg(Color::Red),
                "/".fg(Color::Rgb(200, 100, 255)),
                "N".fg(Color::Green),
                ")".fg(Color::Rgb(200, 100, 255)),
            ]))
            .style(Style::default().fg(Color::Red))
            .block(exiting_block);
            frame.render_widget(Clear, exiting_chunk);
            frame.render_widget(exiting, exiting_chunk);
        }
        Mode::Settings => {
            let settings_chunk = Layout::default()
                .direction(Vertical)
                .constraints([
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ])
                .split(centered_rect(60, 60, frame.area()));
            let master_tab: u8 = (app.master_tab_selection % 3) as u8;
            let temperature_tab_n:u8 =(app.settings_tab_1_selection % 2) as u8;
            let wind_speed_tab_n:u8 =(app.settings_tab_2_selection % 4) as u8;
            let precipitation_tab_n:u8 =(app.settings_tab_3_selection % 2) as u8;
            let temperature_tab = Tabs::new([
                    "Celsius",
                    "Farenchite"
                ])
            match master_tab {
                0 => {
                    //making blocks
                    //first block will be showing temperature
                    let selected_block = selected_block("temperature", &colors);
                    //second block will be showing wind speed
                    let second_block = unselected_block("wind speed");
                    // this block will be displaying precipitation
                    let third_block = unselected_block("precipitation");

                }
                1 => {}
                2 => {}
                // this won't happen
                _ => {
                    ratatui::restore();
                    panic!("error while showing settings popup")
                }
            }
        }
        _ => {}
    }
}
//popup Rect generator (this week warning is ment to be there )
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
fn selected_block<'a>(title: &'a str, colors: &'a ColorPalette) -> Block<'a> {
    Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::HeavyTripleDashed)
        .style(Style::default().bg(colors.bg))
}
fn unselected_block(title: &str) -> Block {
    Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::LightTripleDashed)
        .style(Style::default().bg(Color::Black))
}

use crate::app::{AppState, Mode};
use crate::getting_weather::PrecipitationUnits;
use crate::getting_weather::TemperatureUnits;
use crate::getting_weather::WindUnits;
use ratatui::Frame;
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::{Color, Style};
use ratatui::symbols::Marker;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Tabs};
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::canvas::Line as CanLine;

struct Point{
    x: f64,
    y: f64,
}
impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self{x, y}
    }
}

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
struct UnitsSymbols {
    temperature: String,
    wind_speed: String,
    precipitation: String,
}
impl UnitsSymbols {
    pub fn get_units(app: &AppState) -> UnitsSymbols {
        let temperature:String;
        let wind_speed:String;
        let precipitation:String;
        match app.units.temperature {
            TemperatureUnits::Celsius => {
                temperature = String::from("°C");
            }
            TemperatureUnits::Fahrenheit => {
                temperature = String::from("°F");
            }
        }
        match app.units.wind {
            WindUnits::Knots => {
                wind_speed = String::from("kt");
            }
            WindUnits::Kmh => {
                wind_speed = String::from("km/h");
            }
            WindUnits::Ms => {
                wind_speed = String::from("m/s");
            }
            WindUnits::Mph => {
                wind_speed = String::from("mi/h");
            }
        }
        match app.units.precipitation {
            PrecipitationUnits::Millimeter => {
                precipitation = String::from("mm");
            }
            PrecipitationUnits::Inch => {
                precipitation = String::from("in");
            }
        }
        UnitsSymbols {
            temperature,
            wind_speed,
            precipitation,
        }
    }
}

pub fn ui(frame: &mut Frame, app: &mut AppState) {
    let unit_symbols = UnitsSymbols::get_units(app);
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
    let wind_speed_chunks = Layout::default()
        .direction(Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(1), Constraint::Fill(1)])
        .split(main_vertical_chunks[0]);
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
    let main: Paragraph;
    let mut main_chunks_secondary = Layout::default()
        .direction(Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(main_chunks[0].clone());
    if let Some(weather) = app.weather.clone() {
        main_chunks_secondary = Layout::default()
            .direction(Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(7),
                Constraint::Fill(1),
            ])
            .split(main_chunks[0].clone());
        let temp = weather.temperature_2m.to_string();
        let rain = (weather.rain + weather.showers).to_string();
        let snow = weather.snowfall.to_string();
        let cloud_cover = weather.cloud_cover.to_string();
        let surface_presser = weather.surface_pressure.to_string();
        let pressure_msl = weather.pressure_msl.to_string();
        let relative_humidity = weather.relative_humidity_2m.to_string();
        let lines = vec![
            Line::from(vec![
                "temperature: ".fg(colors.fg),
                temp.fg(colors.fg),
                unit_symbols.temperature.clone().fg(colors.fg),
            ])
            .centered(),
            Line::from(vec![
                "rain: ".fg(colors.fg),
                rain.fg(colors.fg),
                unit_symbols.precipitation.clone().fg(colors.fg),
            ])
            .centered(),
            Line::from(vec![
                "snowfall: ".fg(colors.fg),
                snow.fg(colors.fg),
                unit_symbols.precipitation.clone().fg(colors.fg),
            ])
            .centered(),
            Line::from(vec![
                "cloud cover: ".fg(colors.fg),
                cloud_cover.fg(colors.fg),
                "%".fg(colors.fg),
            ])
            .centered(),
            Line::from(vec![
                "surface pressure: ".fg(colors.fg),
                surface_presser.fg(colors.fg),
                "hPa".fg(colors.fg),
            ])
            .centered(),
            Line::from(vec![
                "sea level pressure ".fg(colors.fg),
                pressure_msl.fg(colors.fg),
                "hPa".fg(colors.fg),
            ])
            .centered(),
            Line::from(vec![
                "relative humidity: ".fg(colors.fg),
                relative_humidity.fg(colors.fg),
                "%".fg(colors.fg),
            ])
            .centered(),
        ];
        main =
            Paragraph::new(Text::from(lines)).style(Style::default().fg(colors.fg).bg(colors.bg));
    } else {
        main = Paragraph::new(Line::from(vec!["No Weather data".into()]).centered())
            .style(Style::default().fg(colors.fg).bg(colors.bg));
    }

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
    // making wind informations
    let wind_spd_str:String;
    let wind_dir_str:String;
    let wind_speed: Paragraph;
    let wind_dir: Paragraph;
    if let Some(weather) = app.weather.clone() {
        wind_dir_str = weather.wind_direction_10m.to_string();
        wind_spd_str = weather.wind_speed_10m.to_string();
        wind_speed = Paragraph::new(
            Line::from(vec![
                "wind speed: ".fg(colors.fg),
                wind_spd_str.fg(colors.fg),
                unit_symbols.wind_speed.clone().fg(colors.fg),
            ])
                .centered(),
        );
        wind_dir = Paragraph::new(
            Line::from(vec![
                "wind dir: ".fg(colors.fg),
                wind_dir_str.fg(colors.fg),
                "°".fg(colors.fg),
            ])
                .centered(),
        )
            .block(wind_direction_block);
    }else {
        wind_speed = Paragraph::new(
            Line::from(vec![
                "No Weather data".fg(colors.fg),
            ])
                .centered()
        );
        wind_dir = Paragraph::new(
            Line::from(vec![
                "No Weather data".fg(colors.fg),
            ])
                .centered()
        )
            .block(wind_direction_block);
    }
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
                    "Settings ".fg(colors.fg),
                    "<W>".fg(Color::Rgb(155, 0, 255)),
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
        Mode::Settings => {
            let cheat_sheet = Paragraph::new(
                Line::from(vec![
                    "Up ".fg(colors.fg),
                    "<J>/<UP>".fg(Color::Rgb(155, 0, 255)),
                    "   ".into(),
                    "Down ".fg(colors.fg),
                    "<K>/<DOWN>".fg(Color::Rgb(155, 0, 255)),
                    "   ".into(),
                    "Left ".fg(colors.fg),
                    "<H>/<LEFT>".fg(Color::Rgb(155, 0, 255)),
                    "   ".into(),
                    "Right ".fg(colors.fg),
                    "<L>/<RIGHT>".fg(Color::Rgb(155, 0, 255)),
                    "   ".into(),
                    "Exit ".fg(colors.fg),
                    "<ESC>".fg(Color::Rgb(155, 0, 255)),
                ])
                .centered(),
            )
            .block(cheat_sheet_block);
            frame.render_widget(cheat_sheet, footer_chunks[1]);
        }
    }
    // some rendering
    render_arrow(frame,main_vertical_chunks[1],0);
    frame.render_widget(wind_block, main_vertical_chunks[0]);
    frame.render_widget(main_block, main_chunks[0]);
    frame.render_widget(main, main_chunks_secondary[1]);
    frame.render_widget(wind_speed, wind_speed_chunks[1]);
    frame.render_widget(wind_dir, main_vertical_chunks[1]);
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
            let rect = centered_rect(30, 30, frame.area());
            let settings_rect = Rect {
                x: rect.x + 1,
                y: rect.y + 1,
                height: rect.height - 2,
                width: rect.width - 2,
            };
            let settings_chunk = Layout::default()
                .direction(Vertical)
                .constraints([
                    Constraint::Fill(3),
                    Constraint::Length(3),
                    Constraint::Fill(2),
                    Constraint::Length(3),
                    Constraint::Fill(2),
                    Constraint::Length(3),
                    Constraint::Fill(3),
                ])
                .split(settings_rect);
            frame.render_widget(Clear, centered_rect(30, 30, frame.area()));
            let master_tab: usize = (app.master_tab_selection % 3) as usize;
            let temperature_tab_n: usize = (app.settings_tab_1_selection % 2) as usize;
            let wind_speed_tab_n: usize = (app.settings_tab_2_selection % 4) as usize;
            let precipitation_tab_n: usize = (app.settings_tab_3_selection % 2) as usize;
            let settings_block = Block::default()
                .borders(Borders::ALL)
                .title("Settings")
                .title_alignment(Alignment::Center)
                .style(Style::default().bg(Color::Rgb(50, 50, 50)));
            frame.render_widget(settings_block, centered_rect(30, 30, frame.area()));
            match master_tab {
                0 => {
                    //making blocks
                    //first block will be showing temperature
                    let selected_block = selected_block("temperature", &colors);
                    //second block will be showing wind speed
                    let second_block = unselected_block("wind speed");
                    // this block will be displaying precipitation
                    let third_block = unselected_block("precipitation");
                    let temperature_tab = Tabs::new(vec!["Celsius", "Fahrenheit"])
                        .select(temperature_tab_n)
                        .block(selected_block)
                        .padding("  ", "  ")
                        .highlight_style(Style::default().magenta().on_black().bold());
                    let wind_speed_tab = Tabs::new(vec!["Knots", "km/h", "m/s", "mph"])
                        .select(wind_speed_tab_n)
                        .block(second_block)
                        .padding("  ", "  ")
                        .highlight_style(Style::default().magenta().on_black().bold());
                    let precipitation_tab = Tabs::new(vec!["Millimeter", "Inch"])
                        .select(precipitation_tab_n)
                        .block(third_block)
                        .padding("  ", "  ")
                        .highlight_style(Style::default().magenta().on_black().bold());
                    frame.render_widget(temperature_tab, settings_chunk[1]);
                    frame.render_widget(wind_speed_tab, settings_chunk[3]);
                    frame.render_widget(precipitation_tab, settings_chunk[5]);
                }
                1 => {
                    let first_block = unselected_block("temperature");
                    let selected_block = selected_block("wind speed", &colors);
                    let third_block = unselected_block("precipitation");
                    let temperature_tab = Tabs::new(vec!["Celsius", "Fahrenheit"])
                        .select(temperature_tab_n)
                        .block(first_block)
                        .padding("  ", "  ")
                        .highlight_style(Style::default().magenta().on_black().bold());
                    let wind_speed_tab = Tabs::new(["Knots", "km/h", "m/s", "mph"])
                        .select(wind_speed_tab_n)
                        .block(selected_block)
                        .padding("  ", "  ")
                        .highlight_style(Style::default().magenta().on_black().bold());
                    let precipitation_tab = Tabs::new(["Millimeter", "Inch"])
                        .select(precipitation_tab_n)
                        .block(third_block)
                        .padding("  ", "  ")
                        .highlight_style(Style::default().magenta().on_black().bold());
                    frame.render_widget(temperature_tab, settings_chunk[1]);
                    frame.render_widget(wind_speed_tab, settings_chunk[3]);
                    frame.render_widget(precipitation_tab, settings_chunk[5]);
                }
                2 => {
                    let first_block = unselected_block("temperature");
                    let second_block = unselected_block("wind speed");
                    let selected_block = selected_block("precipitation", &colors);
                    let temperature_tab = Tabs::new(vec!["Celsius", "Fahrenheit"])
                        .select(temperature_tab_n)
                        .block(first_block)
                        .padding("  ", "  ")
                        .highlight_style(Style::default().magenta().on_black().bold());
                    let wind_speed_tab = Tabs::new(["Knots", "km/h", "m/s", "mph"])
                        .select(wind_speed_tab_n)
                        .block(second_block)
                        .padding("  ", "  ")
                        .highlight_style(Style::default().magenta().on_black().bold());
                    let precipitation_tab = Tabs::new(["Millimeter", "Inch"])
                        .select(precipitation_tab_n)
                        .block(selected_block)
                        .padding("  ", "  ")
                        .highlight_style(Style::default().magenta().on_black().bold());
                    frame.render_widget(temperature_tab, settings_chunk[1]);
                    frame.render_widget(wind_speed_tab, settings_chunk[3]);
                    frame.render_widget(precipitation_tab, settings_chunk[5]);
                }
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
fn unselected_block(title: &str) -> Block<'_> {
    Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::LightTripleDashed)
        .style(Style::default().bg(Color::Black))
}
fn render_arrow(frame:&mut  Frame, area: Rect, heading:u16){
    let p1:Point;
    let p2:Point;
    let p3:Point;
    //if heading <= 315 || heading >= 45 {
        p1 = Point::new(3200.0,4000.0);
        p2 = Point::new(2560.0,500.0);
        p3 = Point::new(3840.0,500.0);
    //}
    let canvas = Canvas::default()
        .x_bounds([0.0,6400.0])
        .y_bounds([0.0,4500.0])
        .marker(Marker::Braille)
        .paint(|ctx| {
            ctx.draw(&CanLine::new(p1.x,p1.y,p2.x,p2.y, Color::Black));
            ctx.draw(&CanLine::new(p2.x,p2.y,p3.x,p3.y, Color::Red));
            ctx.draw(&CanLine::new(p3.x,p3.y,p1.x,p1.y, Color::White));
        });
    frame.render_widget(canvas, area);
}
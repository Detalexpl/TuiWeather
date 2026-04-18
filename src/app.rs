use crate::downloading::{downloading_data, getting_path};
use crate::getting_location::{Location, get_location};
pub use crate::getting_weather::{
    Current, PrecipitationUnits, TemperatureUnits, Units, WindUnits, get_url, get_weather,
};
use crate::ui::{to_small_frame, ui};
use battery::Manager;
use chrono::{DateTime, Local};
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{self, Event};
use std::path::PathBuf;

#[derive(Debug)]
pub enum Mode {
    Normal,
    Typing,
    Exiting,
    Settings,
}
#[derive(Debug)]
pub struct AppState {
    pub location_input: String,
    pub location: String,
    pub mode: Mode,
    pub valid_location: Option<Location>,
    pub path: PathBuf,
    pub weather: Option<Current>,
    pub battery: Vec<f32>,
    pub last_char: char,
    pub real_time: DateTime<Local>,
    pub units: Units,
    pub master_tab_selection: u128,
    pub settings_tab_1_selection: u128,
    pub settings_tab_2_selection: u128,
    pub settings_tab_3_selection: u128,
}
impl AppState {
    pub fn new() -> Result<Self, String> {
        let battery =
            get_battery_level().map_err(|e| format!("Failed to get battery level: {}", e))?;
        if let Some(path) = getting_path() {
            Ok(AppState {
                location_input: String::new(),
                location: String::new(),
                mode: Mode::Normal,
                valid_location: None,
                path,
                weather: None,
                battery,
                last_char: ' ', //tester: true,
                real_time: Local::now(),
                units: Units::defaults(),
                master_tab_selection: 3,
                settings_tab_1_selection: 2,
                settings_tab_2_selection: 4,
                settings_tab_3_selection: 2,
            })
        } else {
            Err("unable to get path".into())
        }
    }
}
pub fn get_battery_level() -> Result<Vec<f32>, String> {
    let batteries = Manager::new().map_err(|_| "battery manager initialization failed")?;
    let mut levels: Vec<f32> = Vec::new();
    for battery in batteries
        .batteries()
        .map_err(|_| "battery manager initialization failed")?
    {
        if let Ok(battery) = battery {
            let percentage = battery.state_of_charge().value * 100.0;
            levels.push(percentage);
        }
    }
    Ok(levels)
}

pub async fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppState) -> Result<(), String> {
    let path_to_cities = app.path.clone().join("cities.csv");
    match path_to_cities.exists() {
        true => {}
        false => {
            downloading_data(&app.path).await?;
        }
    }

    loop {
        app.real_time = Local::now();
        terminal
            .draw(|f| {
                if f.area().height > 37 && f.area().width > 136 {
                    ui(f, app)
                } else {
                    to_small_frame(f)
                }
            })
            .map_err(|err| err.to_string())?;
        //app.tester = false;
        if let Event::Key(key) = event::read().map_err(|_| "Unable to get key event".to_string())? {
            if key.kind == KeyEventKind::Release {
                continue;
            } else {
                if let Some(char) = key.code.as_char() {
                    app.last_char = char;
                }
            }

            match app.mode {
                Mode::Typing => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Esc => {
                                app.location_input.clear();
                                app.mode = Mode::Normal;
                            }
                            KeyCode::Backspace => {
                                app.location_input.pop();
                            }
                            KeyCode::Enter => {
                                app.valid_location =
                                    get_location(&path_to_cities, &app.location_input)
                                        .map_err(|_| "Unable to get location".to_string())?;
                                if let Some(_) = &app.valid_location {
                                    let url = get_url(&app)
                                        .await
                                        .map_err(|_| "Unable to get url".to_string())?;

                                    if let Ok(weather) = get_weather(url).await{
                                        app.weather = Some(weather.current.unwrap());
                                    } else {
                                        app.weather = None;
                                    }

                                    app.location = app.location_input.clone()
                                }
                                app.location_input.clear();
                                app.mode = Mode::Normal;
                            }
                            KeyCode::Char(value) => {
                                app.location_input.push(value);
                            }
                            _ => {}
                        }
                    }
                }
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        app.mode = Mode::Exiting;
                    }
                    KeyCode::Char('s') => {
                        app.mode = Mode::Typing;
                    }
                    KeyCode::Char('r') => {
                        match &app.valid_location {
                            Some(_) => {
                                let url= get_url(&app).await.map_err(|_| "Unable to get url".to_string())?;
                                    if let Ok(weather) = get_weather(url).await {
                                        app.weather = Some(weather.current.unwrap());
                                    } else {
                                        app.weather = None;
                                    }

                            }
                            None => {}
                        }

                        app.battery = get_battery_level()
                            .map_err(|_| "unable to get battery info".to_string())?;
                    }
                    KeyCode::Char('w') => {
                        app.mode = Mode::Settings;
                    }
                    _ => {}
                },
                Mode::Exiting => match key.code {
                    KeyCode::Char('y') => break Ok(()),
                    KeyCode::Char('n') => {
                        app.mode = Mode::Normal;
                    }
                    _ => {}
                },
                Mode::Settings => {
                    match key.code {
                        KeyCode::Char('j') | KeyCode::Up => {
                            app.master_tab_selection = app.master_tab_selection + 2;
                        }
                        KeyCode::Char('k') | KeyCode::Down => {
                            app.master_tab_selection = app.master_tab_selection + 1;
                        }
                        KeyCode::Char('h') | KeyCode::Left => {
                            let tab = app.master_tab_selection % 3;
                            match tab {
                                0 => {
                                    app.settings_tab_1_selection = app.settings_tab_1_selection + 1;
                                }
                                1 => {
                                    app.settings_tab_2_selection = app.settings_tab_2_selection + 3;
                                }
                                2 => {
                                    app.settings_tab_3_selection = app.settings_tab_3_selection + 1;
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Char('l') | KeyCode::Right => {
                            let tab = app.master_tab_selection % 3;
                            match tab {
                                0 => {
                                    app.settings_tab_1_selection = app.settings_tab_1_selection + 1;
                                }
                                1 => {
                                    app.settings_tab_2_selection = app.settings_tab_2_selection + 1;
                                }
                                2 => {
                                    app.settings_tab_3_selection = app.settings_tab_3_selection + 1;
                                }
                                _ => {}
                            }
                        }
                        KeyCode::Esc => {
                            app.mode = Mode::Normal;
                        }
                        _ => {}
                    }
                    let temperature_selection = (app.settings_tab_1_selection % 2) as usize;
                    let wind_speed_selection = (app.settings_tab_2_selection % 4) as usize;
                    let precipitation_selection = (app.settings_tab_3_selection % 2) as usize;
                    let temperature_unit: TemperatureUnits;
                    let wind_speed_unit: WindUnits;
                    let precipitation_unit: PrecipitationUnits;
                    match temperature_selection {
                        0 => {
                            temperature_unit = TemperatureUnits::Celsius;
                        }
                        1 => {
                            temperature_unit = TemperatureUnits::Fahrenheit;
                        }
                        _ => {
                            temperature_unit = TemperatureUnits::Celsius;
                        }
                    }
                    match wind_speed_selection {
                        0 => wind_speed_unit = WindUnits::Knots,
                        1 => wind_speed_unit = WindUnits::Kmh,
                        2 => wind_speed_unit = WindUnits::Ms,
                        3 => wind_speed_unit = WindUnits::Mph,
                        _ => wind_speed_unit = WindUnits::Knots,
                    }
                    match precipitation_selection {
                        0 => precipitation_unit = PrecipitationUnits::Millimeter,
                        1 => precipitation_unit = PrecipitationUnits::Inch,
                        _ => precipitation_unit = PrecipitationUnits::Millimeter,
                    }
                    let selected_units = Units {
                        wind: wind_speed_unit,
                        temperature: temperature_unit,
                        precipitation: precipitation_unit,
                    };
                    app.units = selected_units;
                }
            }
        }
    }
}

use crate::downloading::{downloading_data, getting_path};
use crate::getting_location::{Location, get_location};
use crate::getting_weather::{Current, get_url, get_weather};
use crate::ui::ui;
use battery::Manager;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui::crossterm::event::{self, Event};
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Mode {
    Normal,
    Typing,
    Exiting,
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
    pub tester:bool,
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
                tester: true,
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
        terminal
            .draw(|mut f| ui(f, app))
            .map_err(|err| err.to_string())?;
        //app.tester = false;
        if let Event::Key(key) = event::read().map_err(|_| "Unable to get key event".to_string())? {
            if key.kind == KeyEventKind::Release {
                continue;
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
                                if let Some(location) = &app.valid_location {
                                    app.weather = get_weather(
                                        get_url(location)
                                            .await
                                            .map_err(|_| "Unable to get location".to_string())?,
                                    )
                                    .await
                                    .unwrap()
                                    .current;
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
                            Some(location) => {
                                if let Ok(url) = get_url(&location).await {
                                    app.weather =
                                        get_weather(url).await.map_err(|e| e.to_string())?.current;
                                } else {
                                    return Err("Unable to get Api url".to_string());
                                }
                            }
                            None => {}
                        }

                        app.battery = get_battery_level().map_err(|_| "unable to get battery info".to_string())?;
                    },
                    _ => {}
                },
                Mode::Exiting => match key.code {
                    KeyCode::Char('y') => break Ok(()),
                    KeyCode::Char('n') => {
                        app.mode = Mode::Normal;
                    }
                    _ => {}
                },
            }
        }

    }
}

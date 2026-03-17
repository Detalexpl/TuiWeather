use crate::downloading::getting_path;
use crate::getting_location::{Location, get_location};
use ratatui::{Terminal};
use ratatui::backend::Backend;
use ratatui::crossterm::event::{Event,self};
use std::path::PathBuf;
use crossterm::event::{KeyCode, KeyEventKind};
use crate::getting_weather::{get_url, get_weather, Current };
use crate::ui::ui;

#[derive(Debug)]
pub enum Mode{
    Normal,
    Typing,
    Exiting,
}
#[derive(Debug)]
pub struct AppState {
    pub location_input: String,
    pub mode:Mode,
    pub valid_location: Option<Location>,
    pub path: PathBuf,
    pub weather: Option<Current>,
}
impl AppState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(path) = getting_path() {
            Ok(AppState {
                location_input: String::new(),
                mode:Mode::Normal,
                valid_location: None,
                path,
                weather: None,

            })
        } else {
            Err("unable to get path".into())
        }
    }
    pub fn validate_location(&mut self) {
        if let Ok(location) = get_location(&self.path, &self.location_input) {
            if let Some(location) = location {
                self.valid_location = Some(location);
            } else {
                self.valid_location = None;
            }
        } else {
            self.valid_location = None;
        }
    }
    pub fn clean_input(&mut self) {
        self.location_input.clear();
    }
}

pub async  fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppState,) -> Result<(), String> {
    loop {
        //match terminal.draw(|f| ui(f, app)) {
          //  Err(e) => return Err(e.to_string()),
            //Ok(_) => {}
        //}
        terminal.draw(|mut f| {ui (f, app)}).map_err(|err| err.to_string())?;
        if let Event::Key(key) = event::read().map_err(|_| "Unable to get key event".to_string())? {
            if key.kind == KeyEventKind::Release{
                continue;
            }
            match app.mode {
                Mode::Typing => if key.kind == KeyEventKind::Press {
                    match key.code{
                        KeyCode::Esc => {
                            app.location_input.clear();
                            app.mode = Mode::Normal;
                        }
                        KeyCode::Backspace => {
                            app.location_input.pop();
                        }
                        KeyCode::Enter => {
                            app.valid_location = get_location(&app.path,&app.location_input).map_err(|_| "Unable to get location".to_string())?;
                            app.location_input.clear();
                            app.mode = Mode::Normal;
                        }
                        KeyCode::Char(value) =>{
                            app.location_input.push(value);
                        }
                        _ =>{}
                    }
                }
                Mode::Normal => {
                    match key.code {
                        KeyCode::Char('q') => {
                            app.mode = Mode::Exiting;

                        },
                        KeyCode::Char('s') => {

                            app.mode = Mode::Typing;
                        },
                        KeyCode::Char('r')=>{
                            match &app.valid_location {
                                Some(location) => {
                                    if let Ok(url) = get_url(&location).await {
                                        app.weather = get_weather(url).await.map_err(|e| e.to_string())?.current;
                                    }
                                    else{
                                        return Err("Unable to get Api url".to_string());
                                    }
                                }
                                None => {}
                            }
                        },
                        _ =>{}
                    }
                },
                Mode::Exiting => {
                    match key.code {
                        KeyCode::Char('y') => {
                            break Ok(())
                        }
                        KeyCode::Char('n') => {
                            app.mode = Mode::Normal;
                        }
                        _ =>{}
                    }
                }
            }
        }
    }
}

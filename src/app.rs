use std::path::PathBuf;
use crate::downloading::getting_path;
use crate::getting_location::{get_location, Location};

struct AppState {
    location_input:String,
    typing: bool,
    valid_location:Option<Location>,
    path:PathBuf,
}
impl AppState {
    fn new() -> Result<Self,Box<dyn std::error::Error>> {
        if let Some(path) = getting_path(){

            Ok(AppState{
                location_input: String::new(),
                typing: false,
                valid_location: None,
                path,
            })
        }else {
            Err("unable to get path".into())
        }
    }
    fn validate_location(&mut self) {
        if let Ok(location) = get_location(&self.path, &self.location_input) {
            if let Some(location) = location {
                self.valid_location = Some(location);
            }else {
                self.valid_location = None;
            }
        }else {
            self.valid_location = None;
        }
    }
}

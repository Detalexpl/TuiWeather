use crate::downloading::*;
use crate::getting_location::*;
use crate::getting_weather::{get_url, get_weather};
use std::path::Path;
use std::io;
pub mod downloading;
pub mod getting_location;
pub mod getting_weather;
mod app;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdin =  String::new();
    io::stdin().read_line(&mut stdin)?;



    let path = getting_path();
    match path {
        Some(path) => {
            if !Path::new(&path).exists() {
                downloading_data(path).await.expect("TODO: panic message");
            };
        }
        None => {
            eprint!("invalid path");
            std::process::exit(1);
        }
    }
    let path = match getting_path() {
        Some(path) => path.join("cities.csv"),
        None => {
            eprint!("unable to get path");
            std::process::exit(1);
        }
    };
    let location = match get_location(&path,stdin.trim()) {
        Ok(location) => match location {
            Some(location) => location,
            None => {
                eprint!("unable to get location");
                std::process::exit(1);
            }
        },
        Err(_) => {
            eprint!("unable to get path");
            std::process::exit(1);

        }
    };

    let url = get_url(location).await?;
    let weather = get_weather(url).await?;
    dbg!(weather);

    Ok(())
}


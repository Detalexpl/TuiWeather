use crate::downloading::*;
use crate::getting_location::*;
use crate::getting_weather::{get_url, get_weather};
use std::path::Path;
pub mod downloading;
pub mod getting_location;
pub mod getting_weather;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let location = Location {
        latitude: 52.22,
        longitude: 21.01,
    };
    let url = get_url(location).await?;
    let weather = get_weather(url).await?;
    dbg!(weather);

    Ok(())
}

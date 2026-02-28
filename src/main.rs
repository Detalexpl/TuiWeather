use std::path::{Path, };
use crate::getting_weather::{get_url, get_weather, Location};
use crate::downloading::*;

pub mod getting_weather;
pub mod downloading;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = getting_path();
    match path {
        Some(path) => {if !Path::new(&path).exists(){
            downloading_data(path).await.expect("TODO: panic message");

        }; }
        None => {eprint!("invalid path"); std::process::exit(1,); }
    }



    let location = Location{
        latitude:52.22,
        longitude:21.01
    };
    let url = get_url(location).await?;
    let weather = get_weather(url).await?;
    //dbg!(weather);


    Ok(())
}

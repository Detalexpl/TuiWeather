use crate::getting_weather::{get_url, get_weather, Location};
use std::io;
use std::fs::File;
use std::io::Write;

pub mod getting_weather;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::Client::new().get("https://raw.githubusercontent.com/Detalexpl/TuiWeather/refs/heads/master/worldcities.csv").send().await?.text().await?;
    let mut file = File::create("path")?;
    file.write_all(resp.as_bytes())?;
    let location = Location{
        latitude:52.22,
        longitude:21.01
    };
    let url = get_url(location).await?;
    let weather = get_weather(url).await?;
    dbg!(weather);


    Ok(())
}

use crate::getting_weather::{get_url, get_weather, Location};

pub mod getting_weather;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let location = Location{
        latitude:52.22,
        longitude:21.01
    };
    let url = get_url(location).await?;
    let weather = get_weather(url).await?;
    dbg!(weather);


    Ok(())
}

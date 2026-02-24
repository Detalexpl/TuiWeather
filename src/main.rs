use serde::{ Serialize, Deserialize };
#[derive(Serialize, Deserialize, Debug)]
struct WeatherResponse {

    current: Current,
}
#[derive(Serialize, Deserialize, Debug)]
struct Current{
    time: String,
    interval:u16,
    temperature_2m: f64,
    is_day: u8,
    rain:f64,
    showers:f64,
    weather_code:u8,
    cloud_cover:u8,
    snowfall:f64,
    pressure_msl:f64,
    surface_pressure:f64,
    wind_speed_10m:f64,
    relative_humidity_2m:u8,
    wind_direction_10m: u16,

    
}
#[tokio::main]
async  fn main()-> Result<(),reqwest::Error> {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=52.22&longitude=21.01&current=temperature_2m,is_day,rain,showers,weather_code,cloud_cover,snowfall,pressure_msl,surface_pressure,wind_speed_10m,relative_humidity_2m,wind_direction_10m&timezone=Europe%2FBerlin";
    let weather : WeatherResponse = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json()
        .await?;
    dbg!("{:#?}", weather);


    Ok(())
}

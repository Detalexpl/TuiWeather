use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherResponse {
    current: Option<Current>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Current {
    pub time: String,
    pub interval: u16,
    pub temperature_2m: f64,
    pub is_day: u8,
    pub rain: f64,
    pub showers: f64,
    pub weather_code: u8,
    pub cloud_cover: u8,
    pub snowfall: f64,
    pub pressure_msl: f64,
    pub surface_pressure: f64,
    pub wind_speed_10m: f64,
    pub relative_humidity_2m: u8,
    pub wind_direction_10m: u16,
}
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}
//this function is yous to create api url
pub async fn get_url(location: Location) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,is_day,rain,showers,weather_code,cloud_cover,snowfall,pressure_msl,surface_pressure,wind_speed_10m,relative_humidity_2m,wind_direction_10m&timezone=auto",
        location.latitude, location.longitude
    );
    Ok(url)
}
pub async fn get_weather(url: String) -> Result<WeatherResponse, reqwest::Error> {
    let weather_response:WeatherResponse = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json()
        .await?;
    Ok(weather_response)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn get_url_test() {
        let location = Location {
            latitude: 52.222,
            longitude: 21.01,
        };
        let url = get_url(location).await.unwrap();

        assert_eq!(
            url,
            "https://api.open-meteo.com/v1/forecast?latitude=52.222&longitude=21.01&current=temperature_2m,is_day,rain,showers,weather_code,cloud_cover,snowfall,pressure_msl,surface_pressure,wind_speed_10m,relative_humidity_2m,wind_direction_10m&timezone=auto"
        );
    }

}

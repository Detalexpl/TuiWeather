use crate::app::AppState;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum TemperatureUnits {
    Celsius,
    Fahrenheit,
}
#[derive(Debug)]
pub enum WindUnits {
    Knots,
    Kmh,
    Ms,
    Mph,
}
#[derive(Debug)]
pub enum PrecipitationUnits {
    Millimeter,
    Inch,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct WeatherResponse {
    pub current: Option<Current>,
}
#[derive(Debug)]
pub struct Units {
    pub temperature: TemperatureUnits,
    pub wind: WindUnits,
    pub precipitation: PrecipitationUnits,
}
impl Units {
    pub fn defaults() -> Units {
        Units {
            temperature: TemperatureUnits::Celsius,
            wind: WindUnits::Knots,
            precipitation: PrecipitationUnits::Millimeter,
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
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

//this function is yous to create api url
pub async fn get_url(app: &AppState) -> Result<String, Box<dyn std::error::Error>> {
    let mut _temperature_unit = String::new();
    match app.units.temperature {
        TemperatureUnits::Celsius => _temperature_unit = String::from("celsius"),
        TemperatureUnits::Fahrenheit => _temperature_unit = String::from("fahrenheit"),
    }
    let mut _wind_speed_unit = String::new();
    match app.units.wind {
        WindUnits::Ms => _wind_speed_unit = String::from("ms"),
        WindUnits::Kmh => _wind_speed_unit = String::from("kmh"),

        WindUnits::Mph => _wind_speed_unit = String::from("mph"),
        WindUnits::Knots => _wind_speed_unit = String::from("kn"),
    }
    let mut _precipitation = String::new();
    match app.units.precipitation {
        PrecipitationUnits::Inch => _precipitation = String::from("inch"),
        PrecipitationUnits::Millimeter => _precipitation = String::from("mm"),
    }
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,is_day,rain,showers,weather_code,cloud_cover,snowfall,pressure_msl,surface_pressure,wind_speed_10m,relative_humidity_2m,wind_direction_10m&timezone=auto&wind_speed_unit={}&temperature_unit={}&precipitation_unit={}",
        app.valid_location.clone().unwrap().latitude,
        app.valid_location.clone().unwrap().longitude,
        _wind_speed_unit,
        _temperature_unit,
        _precipitation
    );
    Ok(url)
}
pub async fn get_weather(url: String) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    let weather_response: WeatherResponse =
        reqwest::Client::new().get(url).send().await?.json().await?;
    Ok(weather_response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::getting_location::Location;
    use macro_rules_attribute::apply;
    use smol_macros::{test};
    #[apply(test!)]
    async fn get_url_test() {
        let mut app_state = AppState::new().unwrap();
        let location = Location {
            latitude: 52.222,
            longitude: 21.01,
        };
        app_state.valid_location = Some(location);
        let url = get_url(&app_state).await.unwrap();

        assert_eq!(
            url,
            "https://api.open-meteo.com/v1/forecast?latitude=52.222&longitude=21.01&current=temperature_2m,is_day,rain,showers,weather_code,cloud_cover,snowfall,pressure_msl,surface_pressure,wind_speed_10m,relative_humidity_2m,wind_direction_10m&timezone=auto&wind_speed_unit=kn&temperature_unit=celsius&precipitation_unit=mm"
        );
    }
}

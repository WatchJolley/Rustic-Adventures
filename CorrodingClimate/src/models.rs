use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GeocodingJSON {
    pub display_name: String,
    pub lat: String,
    pub lon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenMetroWeatherJSON {
    pub temperature: f64,
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenMetroJSON {
    pub current_weather: OpenMetroWeatherJSON,
}

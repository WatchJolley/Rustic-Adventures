mod api;
mod models;

use crate::api::urls;
use models::{GeocodingJSON, OpenMetroJSON};
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct Cache<T> {
    cahce_type: T,
    data: HashMap<String, T>,
}

async fn get_openmetro_weather(lat: &str, long: &str) -> Result<OpenMetroJSON, reqwest::Error> {
    let openmetro_url = format!(
        "{}?latitude={}&longitude={}&current_weather=true&temperature_unit=celsius",
        urls::OPENMETRO_URL,
        lat,
        long
    );
    let openmetro_client = reqwest::Client::new();
    let openmetro_response = openmetro_client
        .get(openmetro_url)
        .send()
        .await?
        .json::<OpenMetroJSON>()
        .await?;

    Ok(openmetro_response)
}

async fn get_geocoding_result(user_location: &str) -> Result<Vec<GeocodingJSON>, reqwest::Error> {
    let geocoding_url = format!("{}?q={}", urls::GEOCODING_URL, user_location);
    let geocoding_client = reqwest::Client::new();
    let geocoding_response = geocoding_client
        .get(geocoding_url)
        .send()
        .await?
        .json::<Vec<GeocodingJSON>>()
        .await?;

    Ok(geocoding_response)
}

fn get_user_input() -> String {
    loop {
        let mut input = String::new();
        print!("Welcome! Please provide your location: ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if !input.is_empty() && input.chars().all(|c| c.is_ascii_alphanumeric() || c == ' ')
                {
                    return input.to_string();
                } else {
                    println!("Invalid input, please try again");
                }
            }
            Err(error) => println!("Error reading input: {}", error),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_input = get_user_input();

    let geocoding_json = get_geocoding_result(&user_input).await?;
    let geocoding_result = geocoding_json.first().unwrap_or_else(|| {
        eprintln!("Sorry! That location could not be found.");
        std::process::exit(1);
    });

    let openmetro_response =
        get_openmetro_weather(&geocoding_result.lat, &geocoding_result.lon).await?;

    println!(
        "Temperature in {}: {}°C",
        geocoding_result.display_name, openmetro_response.current_weather.temperature
    );

    Ok(())
}

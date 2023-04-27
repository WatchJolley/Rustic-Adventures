mod api;
mod models;

use crate::api::urls;
use bincode;
use models::{GeocodingJSON, OpenMetroJSON};
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Coordinates {
    long: String,
    lat: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Cache<T: Serialize> {
    name: String,
    data: HashMap<u64, T>,
}

impl<T: Serialize> Cache<T> {
    pub fn new(name: &str) -> Self {
        // TODO: Validate that the name would always be able to be a the name of
        // a file as it is only used to save the binary cache  as of now.
        Cache {
            name: name.to_string(),
            data: HashMap::new(),
        }
    }

    pub fn hash(key: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    pub fn insert(&mut self, key: &str, value: T) -> Option<T> {
        self.data.insert(Self::hash(key), value)
    }

    pub fn remove(&mut self, key: &str) -> Option<T> {
        self.data.remove(&Self::hash(key))
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.data.get(&Self::hash(key))
    }
}

impl<T: Serialize> Drop for Cache<T> {
    fn drop(&mut self) {
        let binary_path = env::current_exe().expect("Failed to get current executable path");
        let mut cache_path = binary_path
            .parent()
            .expect("Failed to get parent directory of binary")
            .join(".cache");
        std::fs::create_dir_all(&cache_path).expect("Failed to create directory");

        let serialized_data: Vec<u8> =
            bincode::serialize(&self.data).expect("Failed to serialize data");
        let filename = format!(".{}.bin", self.name);
        let file_path = cache_path.join(&filename);
        if let Ok(mut file) = OpenOptions::new().write(true).create(true).open(&file_path) {
            file.write_all(&serialized_data)
                .expect("Failed to write to file");
        }
    }
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
    let mut coordinate_cache: Cache<Coordinates> = Cache::new("coordinates");
    let mut temperature_cache: Cache<f64> = Cache::new("temperatures");

    let user_input: String = get_user_input();

    let geocoding_json = get_geocoding_result(&user_input).await?;
    let geocoding_result = geocoding_json.first().unwrap_or_else(|| {
        eprintln!("Sorry! That location could not be found.");
        std::process::exit(1);
    });

    let user_coordinates: Coordinates = Coordinates {
        lat: geocoding_result.lat.clone(),
        long: geocoding_result.lon.clone(),
    };

    coordinate_cache.insert(&user_input.clone(), user_coordinates.clone());

    let openmetro_response =
        get_openmetro_weather(&geocoding_result.lat, &geocoding_result.lon).await?;

    println!(
        "Temperature in {}: {}°C",
        geocoding_result.display_name, openmetro_response.current_weather.temperature
    );

    temperature_cache.insert(
        &geocoding_result.display_name.clone(),
        openmetro_response.current_weather.temperature.clone(),
    );

    println!("{:#?}", coordinate_cache);
    println!("{:#?}", temperature_cache);

    Ok(())
}

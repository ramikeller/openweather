use serde::Deserialize;

#[derive(Deserialize)]
struct GeoResponse {
    results: Option<Vec<GeoResult>>,
}

#[derive(Deserialize)]
struct GeoResult {
    name: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize)]
struct WeatherResponse {
    current: Current,
}

#[derive(Deserialize)]
struct Current {
    temperature_2m: f64,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <city>", args[0]);
        std::process::exit(1);
    }
    let city = &args[1];

    let geo_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json",
        city.replace(' ', "+")
    );

    let geo_response = match reqwest::blocking::get(&geo_url) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: Failed to connect to geocoding API: {}", e);
            std::process::exit(1);
        }
    };

    if !geo_response.status().is_success() {
        eprintln!("Error: Geocoding API returned status {}", geo_response.status());
        std::process::exit(1);
    }

    let geo: GeoResponse = match geo_response.json() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error: Failed to parse geocoding response: {}", e);
            std::process::exit(1);
        }
    };

    let location = match geo.results.and_then(|mut r| if r.is_empty() { None } else { Some(r.remove(0)) }) {
        Some(l) => l,
        None => {
            eprintln!("Error: City '{}' not found", city);
            std::process::exit(1);
        }
    };

    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m",
        location.latitude, location.longitude
    );

    let weather_response = match reqwest::blocking::get(&weather_url) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: Failed to connect to weather API: {}", e);
            std::process::exit(1);
        }
    };

    if !weather_response.status().is_success() {
        eprintln!("Error: Weather API returned status {}", weather_response.status());
        std::process::exit(1);
    }

    let weather: WeatherResponse = match weather_response.json() {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Error: Failed to parse weather response: {}", e);
            std::process::exit(1);
        }
    };

    println!("Temperature in {}: {:.1}°C", location.name, weather.current.temperature_2m);
}

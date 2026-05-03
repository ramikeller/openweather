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

pub struct WeatherInfo {
    pub city: String,
    pub temperature_c: f64,
}

pub fn fetch_weather(city: &str) -> Result<WeatherInfo, String> {
    let geo_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json",
        city.replace(' ', "+")
    );

    let geo_response = reqwest::blocking::get(&geo_url)
        .map_err(|e| format!("Failed to connect to geocoding API: {}", e))?;

    if !geo_response.status().is_success() {
        return Err(format!("Geocoding API returned status {}", geo_response.status()));
    }

    let geo: GeoResponse = geo_response.json()
        .map_err(|e| format!("Failed to parse geocoding response: {}", e))?;

    let location = geo
        .results
        .and_then(|mut r| if r.is_empty() { None } else { Some(r.remove(0)) })
        .ok_or_else(|| format!("City '{}' not found", city))?;

    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m",
        location.latitude, location.longitude
    );

    let weather_response = reqwest::blocking::get(&weather_url)
        .map_err(|e| format!("Failed to connect to weather API: {}", e))?;

    if !weather_response.status().is_success() {
        return Err(format!("Weather API returned status {}", weather_response.status()));
    }

    let weather: WeatherResponse = weather_response.json()
        .map_err(|e| format!("Failed to parse weather response: {}", e))?;

    Ok(WeatherInfo {
        city: location.name,
        temperature_c: weather.current.temperature_2m,
    })
}

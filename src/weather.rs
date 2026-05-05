use reqwest::blocking::Client;
use serde::Deserialize;
use urlencoding::encode;

const WEATHER_API: &str = "https://api.open-meteo.com/v1/forecast";
const GEOCODING_API: &str = "https://geocoding-api.open-meteo.com/v1/search";

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
    relative_humidity_2m: u8,
}

pub struct WeatherInfo {
    pub city: String,
    pub temperature_c: f64,
    pub humidity_percentage: u8,
}

fn fetch_weather_at(client: &Client, lat: f64, lng: f64, name: String) -> Result<WeatherInfo, String> {
    let weather_url = format!(
        "{WEATHER_API}?latitude={lat}&longitude={lng}&current=temperature_2m,relative_humidity_2m"
    );

    let weather_response = client.get(&weather_url).send()
        .map_err(|e| format!("Failed to connect to weather API: {}", e))?;

    if !weather_response.status().is_success() {
        return Err(format!("Weather API returned status {}", weather_response.status()));
    }

    let weather: WeatherResponse = weather_response.json()
        .map_err(|e| format!("Failed to parse weather response: {}", e))?;

    Ok(WeatherInfo {
        city: name,
        temperature_c: weather.current.temperature_2m,
        humidity_percentage: weather.current.relative_humidity_2m,
    })
}

fn geocode_city(client: &Client, city: &str) -> Result<GeoResult, String> {
    let geo_url = format!(
        "{GEOCODING_API}?name={}&count=1&language=en&format=json",
        encode(city)
    );

    let geo_response = client.get(&geo_url).send()
        .map_err(|e| format!("Failed to connect to geocoding API: {}", e))?;

    if !geo_response.status().is_success() {
        return Err(format!("Geocoding API returned status {}", geo_response.status()));
    }

    let geo: GeoResponse = geo_response.json()
        .map_err(|e| format!("Failed to parse geocoding response: {}", e))?;

    geo.results
        .and_then(|r| r.into_iter().next())
        .ok_or_else(|| format!("City '{}' not found", city))
}

pub fn fetch_weather_city(client: &Client, city: &str) -> Result<WeatherInfo, String> {
    let location = geocode_city(client, city)?;
    fetch_weather_at(client, location.latitude, location.longitude, location.name)
}

fn validate_coords(lat: f64, lng: f64) -> Result<(), String> {
    if !(-90.0..=90.0).contains(&lat) {
        return Err(format!("Invalid latitude '{}': must be between -90 and 90", lat));
    }
    if !(-180.0..=180.0).contains(&lng) {
        return Err(format!("Invalid longitude '{}': must be between -180 and 180", lng));
    }
    Ok(())
}

pub fn fetch_weather_coords(client: &Client, lat: f64, lng: f64) -> Result<WeatherInfo, String> {
    validate_coords(lat, lng)?;
    let name = format!("{}, {}", lat, lng);
    fetch_weather_at(client, lat, lng, name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_lat_too_high() {
        assert!(validate_coords(91.0, 0.0).is_err());
    }

    #[test]
    fn rejects_lat_too_low() {
        assert!(validate_coords(-91.0, 0.0).is_err());
    }

    #[test]
    fn rejects_lng_too_high() {
        assert!(validate_coords(0.0, 181.0).is_err());
    }

    #[test]
    fn rejects_lng_too_low() {
        assert!(validate_coords(0.0, -181.0).is_err());
    }

    #[test]
    fn accepts_boundary_lat_lng() {
        assert!(validate_coords(90.0, 180.0).is_ok());
        assert!(validate_coords(-90.0, -180.0).is_ok());
    }
}

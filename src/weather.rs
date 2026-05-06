use reqwest::blocking::Client;
use serde::Deserialize;
use thiserror::Error;
use urlencoding::encode;
use std::time::Duration;

const WEATHER_API: &str = "https://api.open-meteo.com/v1/forecast";
const GEOCODING_API: &str = "https://geocoding-api.open-meteo.com/v1/search";

#[derive(Error, Debug)]
pub enum WeatherError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API returned status {0}")]
    ApiStatus(reqwest::StatusCode),
    #[error("Failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("City '{0}' not found")]
    NotFound(String),
    #[error("Invalid coordinates: {0}")]
    InvalidCoords(String),
    #[error("Missing data in response: {0}")]
    MissingData(String),
}

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
    current: Option<Current>,
}

#[derive(Deserialize)]
struct Current {
    temperature_2m: f64,
    relative_humidity_2m: u8,
}

#[derive(Debug, Clone)]
pub struct WeatherInfo {
    pub city: String,
    pub temperature_c: f64,
    pub humidity_percentage: u8,
}

pub struct WeatherClient {
    client: Client,
}

impl WeatherClient {
    pub fn new(timeout: Duration) -> Result<Self, reqwest::Error> {
        let client = Client::builder().timeout(timeout).build()?;
        Ok(Self { client })
    }

    /// Fetch the current weather for a specific latitude/longitude and return it as `WeatherInfo`.
    fn fetch_weather_at(&self, lat: f64, lng: f64, name: String) -> Result<WeatherInfo, WeatherError> {
        let weather_url = format!(
            "{WEATHER_API}?latitude={lat}&longitude={lng}&current=temperature_2m,relative_humidity_2m"
        );

        let weather_response = self.client.get(&weather_url).send()?;

        if !weather_response.status().is_success() {
            return Err(WeatherError::ApiStatus(weather_response.status()));
        }

        let weather: WeatherResponse = weather_response.json()?;
        let current = weather.current.ok_or_else(|| WeatherError::MissingData("current weather data".to_string()))?;

        Ok(WeatherInfo {
            city: name,
            temperature_c: current.temperature_2m,
            humidity_percentage: current.relative_humidity_2m,
        })
    }

    /// Look up geographic coordinates for a city name using the geocoding API.
    fn geocode_city(&self, city: &str) -> Result<GeoResult, WeatherError> {
        let geo_url = format!(
            "{GEOCODING_API}?name={}&count=1&language=en&format=json",
            encode(city)
        );

        let geo_response = self.client.get(&geo_url).send()?;

        if !geo_response.status().is_success() {
            return Err(WeatherError::ApiStatus(geo_response.status()));
        }

        let geo: GeoResponse = geo_response.json()?;
        geo.results
            .and_then(|r: Vec<GeoResult>| r.into_iter().next())
            .ok_or_else(|| WeatherError::NotFound(city.to_string()))
    }

    /// Fetch weather by city name by first geocoding the city and then querying weather data.
    pub fn fetch_weather_city(&self, city: &str) -> Result<WeatherInfo, WeatherError> {
        let location = self.geocode_city(city)?;
        self.fetch_weather_at(location.latitude, location.longitude, location.name)
    }

    /// Ensure latitude and longitude values are within valid geographic bounds.
    pub fn validate_coords(lat: f64, lng: f64) -> Result<(), WeatherError> {
        if !(-90.0..=90.0).contains(&lat) {
            return Err(WeatherError::InvalidCoords(format!("latitude '{}' must be between -90 and 90", lat)));
        }
        if !(-180.0..=180.0).contains(&lng) {
            return Err(WeatherError::InvalidCoords(format!("longitude '{}' must be between -180 and 180", lng)));
        }
        Ok(())
    }

    /// Fetch weather directly for a latitude/longitude pair after validating coordinates.
    pub fn fetch_weather_coords(&self, lat: f64, lng: f64) -> Result<WeatherInfo, WeatherError> {
        Self::validate_coords(lat, lng)?;
        let name = format!("{}, {}", lat, lng);
        self.fetch_weather_at(lat, lng, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_lat_too_high() {
        assert!(WeatherClient::validate_coords(91.0, 0.0).is_err());
    }

    #[test]
    fn rejects_lat_too_low() {
        assert!(WeatherClient::validate_coords(-91.0, 0.0).is_err());
    }

    #[test]
    fn rejects_lng_too_high() {
        assert!(WeatherClient::validate_coords(0.0, 181.0).is_err());
    }

    #[test]
    fn rejects_lng_too_low() {
        assert!(WeatherClient::validate_coords(0.0, -181.0).is_err());
    }

    #[test]
    fn accepts_boundary_lat_lng() {
        assert!(WeatherClient::validate_coords(90.0, 180.0).is_ok());
        assert!(WeatherClient::validate_coords(-90.0, -180.0).is_ok());
    }
}

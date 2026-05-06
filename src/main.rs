mod weather;

use clap::Parser;
use std::time::Duration;
use reqwest::blocking::Client;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Timeout in seconds for the HTTP client.
    #[arg(long, default_value_t = 10)]
    timeout: u64,

    /// City name or latitude/longitude values.
    #[arg(value_name = "CITY_OR_LAT")]
    positional: Vec<String>,
}

fn build_client(timeout: Duration) -> Result<Client, reqwest::Error> {
    Client::builder().timeout(timeout).build()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let timeout_secs = args.timeout;

    let client = build_client(Duration::from_secs(timeout_secs))?;
    let weather_client = weather::WeatherClient::new(client);

    let info = match args.positional.len() {
        1 => weather_client.fetch_weather_city(&args.positional[0])?,
        2 => {
            let lat = args.positional[0].parse::<f64>()
                .map_err(|_| format!("Invalid latitude '{}': expected a number", args.positional[0]))?;
            let lng = args.positional[1].parse::<f64>()
                .map_err(|_| format!("Invalid longitude '{}': expected a number", args.positional[1]))?;
            weather_client.fetch_weather_coords(lat, lng)?
        }
        _ => {
            eprintln!("Usage: {} [--timeout <secs>] <city> or {} [--timeout <secs>] <lat> <lng>", 
                     std::env::args().next().unwrap_or_else(|| "openweather".to_string()),
                     std::env::args().next().unwrap_or_else(|| "openweather".to_string()));
            std::process::exit(1);
        }
    };

    println!("Temperature in {}: {:.1}°C, humidity: {}%", info.city, info.temperature_c, info.humidity_percentage);
    Ok(())
}

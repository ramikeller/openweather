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

fn main() {
    let args = Args::parse();
    let timeout_secs = args.timeout;
    let positional = args.positional;

    // Client stub that is shared for both the geocoding and forecasting API calls.
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .unwrap_or_else(|e| {
            eprintln!("Error: failed to build HTTP client: {}", e);
            std::process::exit(1);
        });

    let program_name = std::env::args().next().unwrap_or_else(|| "openweather".to_string());
    let result = match positional.len() {
        1 => weather::fetch_weather_city(&client, &positional[0]),
        2 => {
            let lat = positional[0].parse::<f64>()
                .map_err(|_| format!("Invalid latitude '{}': expected a number", positional[0]));
            let lng = positional[1].parse::<f64>()
                .map_err(|_| format!("Invalid longitude '{}': expected a number", positional[1]));
            match (lat, lng) {
                (Ok(lat), Ok(lng)) => weather::fetch_weather_coords(&client, lat, lng),
                (Err(e), _) | (_, Err(e)) => Err(e),
            }
        }
        _ => {
            eprintln!("Usage:");
            eprintln!("  {} [--timeout <secs>] <city>", program_name);
            eprintln!("  {} [--timeout <secs>] <lat> <lng>", program_name);
            std::process::exit(1);
        }
    };

    match result {
        Ok(info) => println!("Temperature in {}: {:.1}°C, humidity: {}%", info.city, info.temperature_c, info.humidity_percentage),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

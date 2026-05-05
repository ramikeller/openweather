mod weather;

use std::time::Duration;
use reqwest::blocking::Client;

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();

    let mut timeout_secs: u64 = 10;
    let mut positional: Vec<String> = Vec::new();
    let mut i = 1;
    while i < raw_args.len() {
        if raw_args[i] == "--timeout" {
            i += 1;
            if i >= raw_args.len() {
                eprintln!("Error: --timeout requires a value");
                std::process::exit(1);
            }
            timeout_secs = raw_args[i].parse::<u64>().unwrap_or_else(|_| {
                eprintln!("Error: --timeout value '{}' is not a valid number of seconds", raw_args[i]);
                std::process::exit(1);
            });
        } else {
            positional.push(raw_args[i].clone());
        }
        i += 1;
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .unwrap_or_else(|e| {
            eprintln!("Error: failed to build HTTP client: {}", e);
            std::process::exit(1);
        });

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
            eprintln!("  {} [--timeout <secs>] <city>", raw_args[0]);
            eprintln!("  {} [--timeout <secs>] <lat> <lng>", raw_args[0]);
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

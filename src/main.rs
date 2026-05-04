mod weather_api;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let result = match args.len() {
        2 => weather_api::fetch_weather_city(&args[1]),
        3 => {
            let lat = args[1].parse::<f64>()
                .map_err(|_| format!("Invalid latitude '{}': expected a number", args[1]));
            let lng = args[2].parse::<f64>()
                .map_err(|_| format!("Invalid longitude '{}': expected a number", args[2]));
            match (lat, lng) {
                (Ok(lat), Ok(lng)) => weather_api::fetch_weather_coords(lat, lng),
                (Err(e), _) | (_, Err(e)) => Err(e),
            }
        }
        _ => {
            eprintln!("Usage:");
            eprintln!("  {} <city>", args[0]);
            eprintln!("  {} <lat> <lng>", args[0]);
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

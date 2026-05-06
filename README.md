# openweather

A command-line tool that reports the current temperature and humidity for a given city or coordinates, in Celsius.

## Usage

```
openweather [--timeout <secs>] <city>
openweather [--timeout <secs>] <lat> <lng>
```

Multi-word city names should be quoted at the shell:

```
openweather "New York"
```

The `--timeout` option sets the HTTP client timeout in seconds (default: 10).

**Example output**

```
Temperature in Zurich: 13.8°C, humidity: 65%
Temperature in 48.8566, 2.3522: 17.2°C, humidity: 72%
```

## Build

Requires Rust (stable). Build with Cargo:

```
cargo build --release
```

The binary will be at `target/release/openweather`.

## Data Sources

Uses two [Open-Meteo](https://open-meteo.com/) APIs. No API key or account required.

- **Geocoding** — resolves the city name to latitude/longitude
- **Forecast** — returns the current temperature and humidity at those coordinates

## Error Handling

All errors are written to stderr and the process exits with code 1. Conditions include wrong argument count, city not found, invalid latitude/longitude values (must be in range -90–90 and -180–180), API connectivity failures, and unexpected response formats.

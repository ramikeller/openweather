# openweather

A command-line tool that reports the current temperature for a given city, in Celsius.

## Usage

```
openweather <city>
```

Multi-word city names should be quoted at the shell:

```
openweather "New York"
```

**Example output**

```
Temperature in London: 13.8°C
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
- **Forecast** — returns the current temperature at those coordinates

## Error Handling

All errors are written to stderr and the process exits with code 1. Conditions include wrong argument count, city not found, API connectivity failures, and unexpected response formats.

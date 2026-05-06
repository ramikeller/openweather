# Functional Specification: openweather CLI

## Purpose

A command-line tool that reports the current temperature and humidity for a given city or geographic coordinates.

## Usage

```
openweather [--timeout <secs>] <city>
openweather [--timeout <secs>] <lat> <lng>
```

**Arguments**

| Argument | Required | Description |
|---|---|---|
| `--timeout <secs>` | No | HTTP client timeout in seconds (default: `10`) |
| `city` | Yes (when using city mode) | City name (e.g. `London`, `New York`) |
| `lat` `lng` | Yes (when using coordinates mode) | Latitude and longitude as decimal numbers |

The CLI accepts either one positional argument for a city name, or two positional arguments for latitude and longitude. Multi-word city names should be quoted at the shell (e.g. `openweather "New York"`).

**Example output**

```
Temperature in Zurich: 13.8°C, humidity: 63%
```

When city mode is used, the city name in the output reflects the canonical name returned by the geocoding API.

## Data Sources

The application uses two Open-Meteo APIs. No API key or account is required.

### 1. Geocoding

Resolves the city name to geographic coordinates.

- **Endpoint:** `https://geocoding-api.open-meteo.com/v1/search`
- **Query parameters:** `name`, `count=1`, `language=en`, `format=json`
- **Used fields:** `results[0].name`, `results[0].latitude`, `results[0].longitude`

### 2. Weather Forecast

Returns current conditions for the resolved coordinates.

- **Endpoint:** `https://api.open-meteo.com/v1/forecast`
- **Query parameters:** `latitude`, `longitude`, `current=temperature_2m,relative_humidity_2m`
- **Used fields:** `current.temperature_2m`, `current.relative_humidity_2m`

Temperature is requested and displayed in Celsius.

## Error Handling

All errors are written to stderr and exit with code 1.

| Condition | Message |
|---|---|
| Wrong number of arguments | `Usage:` plus auto-generated usage text |
| City not found (empty geocoding results) | `Error: City '<input>' not found` |
| Invalid latitude/longitude parse | `Error: Invalid latitude '<input>': expected a number` or `Error: Invalid longitude '<input>': expected a number` |
| Latitude/longitude out of range | `Invalid latitude '<value>': must be between -90 and 90` or `Invalid longitude '<value>': must be between -180 and 180` |
| Geocoding API unreachable | `Error: Failed to connect to geocoding API: <detail>` |
| Geocoding API non-2xx response | `Error: Geocoding API returned status <code>` |
| Weather API unreachable | `Error: Failed to connect to weather API: <detail>` |
| Weather API non-2xx response | `Error: Weather API returned status <code>` |
| Unexpected response format | `Error: Failed to parse geocoding/weather response: <detail>` |

On success the process exits with code 0.

## Behavior Notes

- City names are URL-encoded for the geocoding request.
- Only the top geocoding result is used (`count=1`). If a city name is ambiguous, the API's highest-ranked match is returned without prompting.
- Coordinates mode bypasses geocoding and uses the provided latitude/longitude directly.
- Temperature is displayed with one decimal place and humidity as an integer percentage.
- The HTTP client timeout can be adjusted with `--timeout`; the default is `10` seconds.

## Out of Scope

- Units other than Celsius
- Multiple cities in a single invocation
- Future forecast temperatures
- Additional weather data beyond current temperature and humidity

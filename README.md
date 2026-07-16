# Weather CLI

> [!NOTE]
> This branch contains the Task 2 implementation. Switch to the `task-3` branch to see the final task and the more complete, database-backed implementation.

## How to run it locally

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- An [OpenWeather API key](https://openweathermap.org/api)

Create a `.env` file in the project root and add your OpenWeather API key:

```dotenv
OPEN_WEATHER_KEY=your_api_key
```

Run the CLI with Cargo:

```sh
cargo run -- <COMMAND>
```

### Commands

#### `fav-city <CITY>`

Display the weather forecast for one of the favorite Belgian cities listed in `data/cities.json`. The city name is matched without regard to letter case.

```sh
cargo run -- fav-city Brussels
```

#### `fav-cities`

Display weather forecasts for all favorite Belgian cities listed in `data/cities.json`.

```sh
cargo run -- fav-cities
```

#### `unknow-belgian-city <CITY>`

Display the weather forecast for a Belgian city that is not listed in `data/cities.json`. The command finds the city through the OpenWeather geocoding API and immediately requests its forecast. Task 2 does not save the city.

```sh
cargo run -- unknow-belgian-city Dinant
```

### Forecast day

By default, each command displays the next available forecast. Use the global `--day` option to select a later day:

```sh
cargo run -- fav-city Brussels --day tomorrow
cargo run -- fav-cities --day day-after-tomorrow
```

Available values are:

- `tomorrow`
- `day-after-tomorrow`

For a selected day, the CLI displays the forecast closest to local noon. Use `--help` to see all commands and options, or `--version` to display the application version:

```sh
cargo run -- --help
cargo run -- --version
```

## Goal

Create a basic CLI that displays weather forecasts for Belgian cities.

## Architecture

### Entity

Represents the application's business concepts and logic:

- `City` and `Coordinate` describe a Belgian city and its location.
- `Forecast` and `CityWeather` represent weather data and select forecasts by local date.

### Weather API

This is the abstraction layer for calls to the external OpenWeather API.

- Data transfer objects (DTOs) represent geocoding and forecast API responses.
- `WeatherClient` sends requests and converts responses into application entities.

### Output

This module represents the terminal user interface. It formats forecast data and renders it as a table containing the city, date, temperature, humidity, and precipitation probability.

### `config.rs`

This module loads the `.env` file and makes the OpenWeather API key available to the application.

### `errors.rs`

This module defines the errors that can occur while loading configuration, reading local data, or calling OpenWeather.

### `app.rs`

This is the application orchestrator. It loads favorite cities from `data/cities.json`, calls OpenWeather, selects the requested forecast, and prepares the terminal output. Requests made by `fav-cities` run concurrently while their displayed order remains consistent with the JSON file.

### `cli.rs`

This module defines the commands and the global `--day` option available to users.

### `main.rs`

This is the application's entry point. It parses the user's command, loads the configuration, delegates work to `app.rs`, and returns the appropriate process exit status.

### `data/cities.json`

This file contains the predefined favorite Belgian cities and their coordinates. Task 2 reads this file directly and does not use a database.

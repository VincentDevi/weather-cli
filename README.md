# Weather CLI

## How to run it locally

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- An [OpenWeather API key](https://openweathermap.org/api)

Create a `.env` file in the project root and add your OpenWeather API key:

```dotenv
OPEN_WEATHER_KEY=your_api_key
```

By default, the application stores its SQLite database at `./data/weather.db`. To use a different location, set the optional `WEATHER_DB_PATH` variable:

```dotenv
WEATHER_DB_PATH=./data/weather.db
```

Run the CLI with Cargo:

```sh
cargo run -- <COMMAND>
```

### Commands

#### `fav-city <CITY>`

Display the weather forecast for a favorite city stored in the database.

```sh
cargo run -- fav-city Brussels
```

#### `fav-cities`

Display weather forecasts for all Belgian cities listed in `data/cities.json`.

```sh
cargo run -- fav-cities
```

#### `unknown-belgian-city <CITY>`

Display the weather forecast for a Belgian city that is not already stored. The command finds the city through OpenWeather and saves it to the database for future requests.

```sh
cargo run -- unknown-belgian-city Dinant
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

Use `--help` to see all commands and options, or `--version` to display the application version:

```sh
cargo run -- --help
cargo run -- --version
```

## Goal

Create a basic CLI that displays weather forecasts for Belgian cities.

## Architecture

### Entity

Represents the application's business logic, type definitions, and ubiquitous language.

### Database and Repository

This is the abstraction layer for all database interactions.

- Models represent database tables.
- Repository functions query and modify data in the database.

### Weather API

This is the abstraction layer for calls to the external OpenWeather API.

- Data transfer objects (DTOs) represent API response models.
- Client functions send requests to the API and convert its responses into application entities.

### Output

This module represents the user interface, including the components and views displayed in the terminal.

### `app.rs`

This is the application orchestrator. It defines the functions called when a user runs a command and delegates work to the appropriate modules.

### `cli.rs`

This module defines the commands and options available to users.

### `main.rs`

This is the application's entry point. It parses the user's command and calls the appropriate function from `app.rs`.

### Tests

End-to-end tests verify the CLI's behavior, including command execution and rendered weather output.

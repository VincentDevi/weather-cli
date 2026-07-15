use clap::Parser;
use cli::*;
use config::Config;
use errors::AppError;
use std::process::ExitCode;

mod app;
mod cli;
mod config;
mod entity;
mod errors;
mod output;
mod weather_api;

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), AppError> {
    let Cli { day, command } = Cli::parse();
    let config = Config::load()?;

    match command {
        Command::FavCity { city } => app::fav_city(config, city, day).await?,
        Command::FavCities => app::fetch_belgian_city_forecasts(config, day).await?,
        Command::UnknowBelgianCity { city } => app::city(config, city, day).await?,
    }

    Ok(())
}

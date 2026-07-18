use clap::Parser;
use std::process::ExitCode;
use weather_cli::{
    app,
    cli::{Cli, Command},
    config::Config,
    database::Database,
    errors::AppError,
};

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
    let mut database = Database::open(config.database_path())?;
    database.initialize()?;

    match command {
        Command::FavCity { city } => {
            app::fav_city(config, &mut database, city, day).await?;
        }
        Command::FavCities => app::fetch_belgian_city_forecasts(config, day).await?,
        Command::UnknownBelgianCity { city } => {
            app::city(config, &mut database, city, day).await?;
        }
        Command::WhatToWearIn { city } => {
            app::suggestion(config, &mut database, city, day).await?;
        }
    }

    Ok(())
}

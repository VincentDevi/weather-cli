use clap::Parser;
use cli::*;
use config::Config;
use errors::AppError;
use std::process::ExitCode;

mod app;
mod cli;
mod config;
mod errors;
mod output;

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
    let cli = Cli::parse();
    let config = Config::load()?;

    match cli.command {
        Command::Fav => app::run(config).await?,
    }

    Ok(())
}

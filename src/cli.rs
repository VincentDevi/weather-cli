use clap::{Parser, Subcommand, ValueEnum};

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ForecastDay {
    Tomorrow,
    DayAfterTomorrow,
}

impl ForecastDay {
    pub fn days_from_now(self) -> u64 {
        match self {
            Self::Tomorrow => 1,
            Self::DayAfterTomorrow => 2,
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "weather", version, about = "weather cli")]
pub struct Cli {
    #[arg(
        long,
        value_enum,
        global = true,
        help = "Show the forecast nearest local noon for a future day"
    )]
    pub day: Option<ForecastDay>,

    #[command(subcommand)]
    pub command: Command,
}
#[derive(Debug, Subcommand)]
pub enum Command {
    FavCity { city: String },
    FavCities,
    UnknownBelgianCity { city: String },
}

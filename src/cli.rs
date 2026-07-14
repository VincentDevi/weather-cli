use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "weather", version, about = "weather cli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Debug, Subcommand)]
pub enum Command {
    Fav,
    List,
}

use super::config::*;
use super::errors::*;

pub async fn run(_config: Config) -> Result<(), AppError> {
    println!("tests");
    Ok(())
}

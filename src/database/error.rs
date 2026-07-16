use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("invalid persisted weather data: {0}")]
    InvalidDatabaseData(String),
    #[error("database I/O error: {0}")]
    DatabaseIo(#[from] std::io::Error),
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("database connection configuration error: {0}")]
    DatabaseConfiguration(String),
    #[error("cannot create or refresh forecasts from an empty response")]
    EmptyForecastResponse,
    #[error("refusing to delete every forecast through an empty filter")]
    EmptyForecastDeleteFilter,
    #[error("{resource} with ID {id} was not found")]
    DatabaseRecordNotFound { resource: &'static str, id: i64 },
    #[error(
        "unsupported database schema version {installed}; this application supports versions 0 through {supported}"
    )]
    UnsupportedDatabaseVersion { installed: i64, supported: i64 },
}

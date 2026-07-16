use std::path::Path;

use rusqlite::{Connection, TransactionBehavior};

mod error;
mod model;
mod repository;

pub use error::DatabaseError;
pub use model::*;
pub use repository::*;

const MIGRATION_1: &str = include_str!("migrations/001_initial.sql");
const LATEST_SCHEMA_VERSION: i64 = 1;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self, DatabaseError> {
        if let Some(parent) = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            std::fs::create_dir_all(parent)?;
        }

        let connection = Connection::open(path)?;
        Self::configure_connection(&connection, true)?;
        Ok(Self { connection })
    }

    pub fn initialize(&mut self) -> Result<(), DatabaseError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let installed_version: i64 =
            transaction.pragma_query_value(None, "user_version", |row| row.get(0))?;

        if !(0..=LATEST_SCHEMA_VERSION).contains(&installed_version) {
            return Err(DatabaseError::UnsupportedDatabaseVersion {
                installed: installed_version,
                supported: LATEST_SCHEMA_VERSION,
            });
        }

        for version in (installed_version + 1)..=LATEST_SCHEMA_VERSION {
            match version {
                1 => transaction.execute_batch(MIGRATION_1)?,
                _ => unreachable!("every schema version must have a migration"),
            }
            transaction.pragma_update(None, "user_version", version)?;
        }

        transaction.commit()?;
        Ok(())
    }

    fn configure_connection(
        connection: &Connection,
        require_wal: bool,
    ) -> Result<(), DatabaseError> {
        connection.pragma_update(None, "foreign_keys", true)?;
        let foreign_keys: bool =
            connection.pragma_query_value(None, "foreign_keys", |row| row.get(0))?;
        if !foreign_keys {
            return Err(DatabaseError::DatabaseConfiguration(
                "SQLite foreign-key enforcement could not be enabled".to_owned(),
            ));
        }

        if require_wal {
            let journal_mode: String =
                connection
                    .pragma_update_and_check(None, "journal_mode", "WAL", |row| row.get(0))?;
            if !journal_mode.eq_ignore_ascii_case("wal") {
                return Err(DatabaseError::DatabaseConfiguration(format!(
                    "SQLite returned journal mode {journal_mode} instead of WAL"
                )));
            }
        }

        Ok(())
    }
}

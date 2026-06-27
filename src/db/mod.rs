pub mod migrations;
pub mod models;
pub mod queries;

use libsql::{Builder, Connection};

/// Database wrapper around libsql (local SQLite mode).
#[derive(Clone)]
pub struct Database {
    db: libsql::Database,
}

impl Database {
    /// Initialize a local SQLite database via libsql.
    /// The database file is auto-created at `./rustcluster.db`.
    pub async fn init() -> Result<Self, libsql::Error> {
        let db = Builder::new_local("rustcluster.db")
            .build()
            .await?;
        Ok(Self { db })
    }

    /// Get a new connection from the database.
    pub fn conn(&self) -> Result<Connection, libsql::Error> {
        self.db.connect()
    }

    /// Run all database migrations.
    pub async fn run_migrations(&self) -> Result<(), libsql::Error> {
        let conn = self.conn()?;
        migrations::run_all(&conn).await
    }
}

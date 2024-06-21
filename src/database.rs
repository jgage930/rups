use anyhow::Result;
use rusqlite::Connection;

/// A trait to allow a struct to interact with a database.
pub trait DbBase {
    // Create table in db.
    fn create_table() -> Result<()>;

    // Insert a new row into db.
    fn insert(&self, conn: &Connection) -> Result<Self>
    where
        Self: Sized;
}

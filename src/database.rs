use anyhow::Result;
use rusqlite::Connection;

/// A trait to allow a struct to interact with a database.
pub trait DbBase {
    // Create table in db.
    fn create_table(conn: &Connection) -> Result<()>;

    // Insert a new row into db.
    fn insert(&self, conn: &Connection) -> Result<Self>
    where
        Self: Sized;
}

pub fn connect() -> Connection {
    Connection::open("data.db").expect("Could not connect to the database.")
}

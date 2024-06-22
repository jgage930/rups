use crate::password::Password;
use anyhow::Result;
use rusqlite::Connection;

/// A trait to allow a struct to interact with a database.
pub trait DbBase {
    // Create table in db.
    fn create_table(conn: &Connection) -> Result<()>;

    // Insert a new row into db.
    fn insert(&self, conn: &Connection) -> Result<i64>;

    /// Get a single row by id
    fn get_by_id(id: i64, conn: &Connection) -> Result<Option<Self>>
    where
        Self: Sized;

    /// List all items in table.
    fn list(conn: &Connection) -> Result<Vec<Self>>
    where
        Self: Sized;
}

pub fn connect() -> Connection {
    Connection::open("data.db").expect("Could not connect to the database.")
}

pub fn setup_db() -> Result<Connection> {
    let conn = connect();

    Password::create_table(&conn)?;

    Ok(conn)
}

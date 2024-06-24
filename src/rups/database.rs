use super::password::Password;
use anyhow::{Result};
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

    fn get_by_col(col: &str, value: &str, conn: &Connection) -> Result<Vec<Self>>
    where
        Self: Sized;

    /// List all items in table.
    fn list(conn: &Connection) -> Result<Vec<Self>>
    where
        Self: Sized;
}

// pub struct TableName(&'static str);
//
// pub struct Column {
//     name: String,
//     type_: String,
//     nullable: bool,
// }

// // This trait defines how a struct should interact with the db.
// pub trait DbModel {
//     fn table_name() -> TableName;
//     fn columns() -> Vec<Column>;
//
//     fn create_table(db: &Connection) -> Result<()> {
//         let col_sql = Self::columns()
//             .iter()
//             .map(|col| {
//                 let null_sql = if col.nullable { "NOT NULL" } else { "" };
//                 format!("{} {} {},", col.name, col.type_, null_sql)
//             })
//             .collect::<Vec<String>>()
//             .join("\n");
//
//         let query = format!(
//             "CREATE TABLE IF NOT EXISTS {} ({})",
//             Self::table_name().0,
//             col_sql
//         );
//
//         db.execute(&query, ())
//             .context("Failed to create the password table")?;
//
//         Ok(())
//     }
//
//     fn get_by_col(col: &str, db: &Connection) -> Result<Vec<Self>> {}
// }

pub fn connect() -> Connection {
    Connection::open("data.db").expect("Could not connect to the database.")
}

pub fn setup_db() -> Result<Connection> {
    let conn = connect();

    Password::create_table(&conn)?;

    Ok(conn)
}

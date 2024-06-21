use crate::database::DbBase;
use anyhow::{Context, Result};
use inquire::Text;
use rusqlite::Connection;

#[derive(Debug)]
pub struct Password {
    name: String,
    site: String,
    password: Vec<u8>,
}

impl DbBase for Password {
    fn create_table(conn: &Connection) -> Result<()> {
        let query = "
            CREATE TABLE passwords (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                site TEXT NOT NULL,
                password BLOB NOT NULL
            );
        ";

        conn.execute(query, ())
            .context("Failed to create the password table")?;

        Ok(())
    }

    fn insert(&self, conn: &Connection) -> Result<i64> {
        conn.execute(
            "
                INSERT INTO passwords
                (name, site, password)
                VALUES (?1, ?2, ?3)
            
            ",
            (self.name, self.site, self.password),
        );

        let id = conn.last_insert_rowid();
        Ok(id)
    }
}

pub fn prompt_for_password() -> Result<Password> {
    let name = Text::new("Enter name for Password: ").prompt()?;
    let site = Text::new("Enter site: ").prompt()?;
    let password = Text::new("Enter Password: ").prompt()?.as_bytes().to_vec();

    Ok(Password {
        name,
        site,
        password,
    })
}

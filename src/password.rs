use crate::{
    database::DbBase,
    encrypt::{decrypt, encrypt, read_key},
};
use anyhow::{Context, Result};
use inquire::Text;
use rusqlite::{params, Connection};

#[derive(Debug)]
pub struct Password {
    name: String,
    site: String,
    password: Vec<u8>,
}

impl Password {
    fn encrypt(&mut self) {
        let key = read_key("key");
        self.password = encrypt(&key, &self.password);
    }

    fn decrypt(&mut self) {
        let key = read_key("key");
        self.password = decrypt(&key, &self.password).expect("Failed to decrypt password.");
    }
}

impl DbBase for Password {
    fn create_table(conn: &Connection) -> Result<()> {
        let query = "
            CREATE TABLE IF NOT EXISTS passwords (
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

    fn insert(&mut self, conn: &Connection) -> Result<i64> {
        self.encrypt();

        conn.execute(
            "   
                INSERT INTO passwords
                (name, site, password)
                VALUES (?1, ?2, ?3)
            
            ",
            (&self.name, &self.site, &self.password),
        )?;

        let id = conn.last_insert_rowid();
        Ok(id)
    }

    fn get_by_id(id: i64, conn: &Connection) -> Result<Option<Self>> {
        let mut query = conn.prepare("SELECT * FROM passwords WHERE id = ?1")?;

        let row = query.query_row(params![id], |row| {
            Ok(Self {
                name: row.get(1)?,
                site: row.get(2)?,
                password: row.get(3)?,
            })
        })?;

        Ok(Some(row))
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

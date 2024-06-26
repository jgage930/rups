use super::{
    database::DbBase,
    encrypt::{decrypt, encrypt, read_key},
};
use anyhow::{Context, Result};
use inquire::{Autocomplete, Text};
use rusqlite::{params, Connection};
use std::{borrow::Cow, rc::Rc};
use tabled::Tabled;

#[derive(Debug)]
pub struct Password {
    name: String,
    site: String,
    password: Vec<u8>,
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

    fn insert(&self, conn: &Connection) -> Result<i64> {
        let key = read_key("key");
        let encrypted_password = encrypt(&key, &self.password);

        conn.execute(
            "   
                INSERT INTO passwords
                (name, site, password)
                VALUES (?1, ?2, ?3)
            
            ",
            (&self.name, &self.site, &encrypted_password),
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

    fn list(conn: &Connection) -> Result<Vec<Self>> {
        let mut query = conn.prepare("SELECT * FROM passwords")?;

        let rows = query.query_map([], |row| {
            Ok(Self {
                name: row.get(1)?,
                site: row.get(2)?,
                password: row.get(3)?,
            })
        })?;

        let passwords: Vec<Password> = rows.map(|row| row.unwrap()).collect();
        Ok(passwords)
    }

    fn get_by_col(col: &str, value: &str, conn: &Connection) -> Result<Vec<Self>> {
        let sql = format!("SELECT * FROM passwords WHERE {} = ?1", col);
        let mut query = conn.prepare(&sql)?;

        let rows = query.query_map([value], |row| {
            Ok(Self {
                name: row.get(1)?,
                site: row.get(2)?,
                password: row.get(3)?,
            })
        })?;

        let passwords: Vec<Password> = rows.map(|row| row.unwrap()).collect();
        Ok(passwords)
    }
}

impl Tabled for Password {
    const LENGTH: usize = 3;

    fn fields(&self) -> Vec<Cow<'_, str>> {
        let key = read_key("key");
        let unencrypted_password =
            decrypt(&key, &self.password).expect("Failed to decrypt password from safe.");

        let password = String::from_utf8(unencrypted_password)
            .expect("Could not encode password to utf-8.")
            .to_owned();

        let mut values = Vec::new();
        values.push(Cow::from(&self.name));
        values.push(Cow::from(&self.site));
        values.push(Cow::Owned(password));

        values
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            Cow::Borrowed("Name"),
            Cow::Borrowed("Site"),
            Cow::Borrowed("Password"),
        ]
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

#[derive(Clone)]
pub struct PasswordCompleter {
    db: Rc<Connection>,
}

impl PasswordCompleter {
    pub fn new(db: Rc<Connection>) -> Self {
        Self { db }
    }
}

impl Autocomplete for PasswordCompleter {
    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> std::prelude::v1::Result<inquire::autocompletion::Replacement, inquire::CustomUserError>
    {
        match highlighted_suggestion {
            Some(suggestion) => {
                let index = input.len();
                let replacement = format!("{}{}", input, &suggestion[index..]);

                Ok(Some(replacement))
            }
            None => Ok(Some(input.to_string())),
        }
    }

    fn get_suggestions(
        &mut self,
        input: &str,
    ) -> std::prelude::v1::Result<Vec<String>, inquire::CustomUserError> {
        let mut query = self
            .db
            .prepare("SELECT name FROM passwords WHERE name LIKE ?1")?;

        let pattern = format!("{input}%");
        let rows = query.query_map(params![pattern], |row| Ok(row.get(0)?))?;

        let mut suggestions = Vec::new();
        for row in rows {
            suggestions.push(row?);
        }

        Ok(suggestions)
    }
}

use anyhow::{Context, Result};
use clap::Parser;
use inquire::Text;
use rusqlite::Connection;
use tabled::Table;

use super::{
    args::Args,
    database::{connect, setup_db, DbBase},
    password::{prompt_for_password, Password, PasswordCompleter},
};

fn add_password(db: &Connection) -> Result<()> {
    let password = prompt_for_password().expect("Could not read input from command line.");

    let id = password
        .insert(&db)
        .context("Failed to insert new password entry")?;

    println!("Inserted password {id}");

    Ok(())
}

pub fn run() -> Result<()> {
    let db = setup_db().expect("Failed to set up database.");
    let args = Args::parse();

    match args {
        Args::Add => {
            let password = prompt_for_password().expect("Could not read input from command line.");

            let id = password
                .insert(&db)
                .context("Failed to insert new password entry")?;

            println!("Inserted password {id}");
        }
        Args::Get { id } => {
            let password = Password::get_by_id(id, &db)?;

            let table = Table::new(password).to_string();
            println!("{table}")
        }
        Args::List => {
            let passwords = Password::list(&db)?;

            let table = Table::new(passwords).to_string();
            println!("{table}")
        }
        Args::Search => {
            let completer = PasswordCompleter::new(db.into());

            let search_val = Text::new("Enter Name: ")
                .with_autocomplete(completer)
                .prompt()?;

            let new_db = connect();
            let passwords = Password::get_by_col("name", &search_val, &new_db)?;

            let table = Table::new(passwords).to_string();
            println!("{table}")
        }
    }

    Ok(())
}

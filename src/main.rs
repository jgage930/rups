pub mod args;
pub mod database;
pub mod encrypt;
pub mod password;

use anyhow::{Context, Result};
use args::Args;
use clap::Parser;
use database::{setup_db, DbBase};
use password::{prompt_for_password, Password};
use tabled::Table;

fn main() -> Result<()> {
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
            println!("not implementes")
        }
    }

    Ok(())
}

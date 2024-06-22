use anyhow::{Context, Result};
use clap::Parser;
use tabled::Table;

use super::{
    args::Args,
    database::{setup_db, DbBase},
    password::{prompt_for_password, Password},
};

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
    }

    Ok(())
}

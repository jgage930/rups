pub mod args;
pub mod database;
pub mod password;

use args::Args;
use clap::Parser;
use password::prompt_for_password;

fn main() {
    let args = Args::parse();

    match args {
        Args::Add => {
            let password = prompt_for_password().expect("Could not read input from command line.");
            dbg!(password);
        }
    }
}

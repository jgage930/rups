pub mod args;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    match args {
        Args::Add => {
            println!("Got add")
        }
    }
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub enum Args {
    /// Add a password to the vault.
    Add,
}

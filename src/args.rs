use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub enum Args {
    #[command(version, about = "Add a new password to the safe.")]
    Add,

    #[command(version, about = "Lookup a password by id.")]
    Get {
        /// Id of the password.
        #[arg(short = 'i', long = "id")]
        id: i64,
    },
}

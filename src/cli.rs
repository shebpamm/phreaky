use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Phreaky", version = "0.1", author = "shebpamm")]
pub struct Cli {

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Serve,
}

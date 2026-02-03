mod cli;
mod server;
mod config;

use clap::Parser;
use crate::cli::Cli;

fn main() {
    let args = Cli::parse();

    match args.command {
        Some(cli::Commands::Serve) => {
            server::start_runtime().unwrap();
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}

mod config;
mod cli;

use clap::Parser;
use crate::config::Config;
use crate::cli::Cli;

fn main() {
    let config = Config::get();

    let args = Cli::parse();

    match args.command {
        Some(cli::Commands::Serve) => {
            println!("Starting server...");
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}

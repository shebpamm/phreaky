mod cli;
mod server;

use clap::Parser;
use crate::cli::Cli;

fn main() {
    let args = Cli::parse();

    match args.command {
        Some(cli::Commands::Serve) => {
            server::serve().unwrap();
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}

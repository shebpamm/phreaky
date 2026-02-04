mod cli;
mod server;
mod config;
mod api;
mod db;
mod riot;
mod client;

use clap::Parser;
use crate::cli::Cli;

fn main() {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .init();

    color_eyre::install().unwrap();

    let args = Cli::parse();

    match args.command {
        Some(cli::Commands::Serve) => {
            server::start_runtime().unwrap();
        }
        Some(cli::Commands::Add(args )) => {
            client::add_account(args.name, args.region).unwrap();
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}

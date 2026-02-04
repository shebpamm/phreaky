use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Phreaky", version = "0.1", author = "shebpamm")]
pub struct Cli {

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Parser, Debug)]
pub struct AddArgs {
    #[arg(help = "Summoner name incl. tag")]
    pub name: String,

    #[arg(help = "Summoner region")]
    pub region: crate::riot::Region,
    
    #[arg(short, long, default_value = "http://localhost:8080")]
    pub url: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Serve,
    Add(AddArgs),
}

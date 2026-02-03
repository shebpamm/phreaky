mod config;
use crate::config::Config;

fn main() {
    let config = Config::get();

    println!("db config: {:?}", config.db);
}

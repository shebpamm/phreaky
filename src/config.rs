use dotenv::dotenv;
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug)]
pub struct Config {
    pub db: DatabaseConfig,    
}

#[derive(Debug)]
pub struct DatabaseConfig {
    pub url: String,
    pub token: String,
}

impl Config {
    pub fn get() -> &'static Config {
        CONFIG.get_or_init(|| {
            dotenv().ok();

            let db_url = std::env::var("TURSO_DATABASE_URL")
                .expect("TURSO_DATABASE_URL must be set in .env file");
            let db_token = std::env::var("TURSO_AUTH_TOKEN")
                .expect("TURSO_AUTH_TOKEN must be set in .env file");

            Config {
                db: DatabaseConfig {
                    url: db_url,
                    token: db_token,
                },
            }
        })
    }
}

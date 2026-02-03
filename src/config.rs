use dotenv::dotenv;
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug)]
pub struct Config {
    pub db: DatabaseConfig,    
    pub server: ServerConfig,
}

#[derive(Debug)]
pub struct DatabaseConfig {
    pub url: String,
    pub token: String,
}

#[derive(Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Config {
    pub fn get() -> &'static Config {
        CONFIG.get_or_init(|| {
            dotenv().ok();

            let db_url = std::env::var("TURSO_DATABASE_URL")
                .expect("TURSO_DATABASE_URL must be set in .env file");
            let db_token = std::env::var("TURSO_AUTH_TOKEN")
                .expect("TURSO_AUTH_TOKEN must be set in .env file");

            let server_host = std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string());

            let server_port = std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse::<u16>()
                .expect("SERVER_PORT must be a valid u16");

            Config {
                db: DatabaseConfig {
                    url: db_url,
                    token: db_token,
                },
                server: ServerConfig {
                    host: server_host,
                    port: server_port,
                },
            }
        })
    }
}

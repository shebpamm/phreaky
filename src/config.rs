use dotenv::dotenv;
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug)]
pub struct Config {
    pub db: DatabaseConfig,    
    pub server: ServerConfig,
    pub riot: RiotConfig,
    pub worker: WorkerConfig,
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

#[derive(Debug)]
pub struct RiotConfig {
    pub token: String,
}

impl ServerConfig {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug)]
pub struct WorkerConfig {
    pub interval_seconds: u64,
}

impl Config {
    pub fn get() -> &'static Config {
        CONFIG.get_or_init(|| {
            dotenv().ok();

            let db_url = std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set in .env file");
            let db_token = std::env::var("DATABASE_TOKEN")
                .expect("DATABASE_TOKEN must be set in .env file");

            let server_host = std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string());

            let server_port = std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse::<u16>()
                .expect("SERVER_PORT must be a valid u16");

            let riot_token = std::env::var("RIOT_API_KEY")
                .expect("RIOT_API_KEY must be set in .env file");

            let interval_seconds = std::env::var("WORKER_INTERVAL_SECONDS")
                .unwrap_or_else(|_| "60".to_string())
                .parse::<u64>()
                .expect("WORKER_INTERVAL_SECONDS must be a valid u64");

            Config {
                db: DatabaseConfig {
                    url: db_url,
                    token: db_token,
                },
                server: ServerConfig {
                    host: server_host,
                    port: server_port,
                },
                riot: RiotConfig {
                    token: riot_token,
                },
                worker: WorkerConfig {
                    interval_seconds,
                },
            }
        })
    }
}

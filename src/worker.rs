use thiserror::Error;

use crate::config::Config;
use crate::db::account;
use crate::riot;

#[derive(Error, Debug)]
pub enum WorkerError {
    #[error("Internal error: {0}")]
    InternalError(#[from] color_eyre::eyre::Report),
    #[error("Account Database error: {0}")]
    AccountDbError(#[from] account::AccountError),
}

pub type Result<T> = std::result::Result<T, WorkerError>;

async fn scrape(account: &account::Account) -> Result<()> {
    let stats = riot::get_player_stats(&account.puuid, &account.region).await?;

    println!("Fetched stats for {}#{}: {:?}", account.username, account.tagline, stats);

    Ok(())
}

pub async fn background_worker() -> Result<()> {
    let config = Config::get();
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(config.worker.interval_seconds));

    println!("Background worker started with interval: {} seconds", config.worker.interval_seconds);

    loop {
        interval.tick().await;

        let accounts = account::list_accounts().await?;

        for account in accounts {
            println!("Processing account: {}#{}", account.username, account.tagline);

            match scrape(&account).await {
                Ok(_) => println!("Successfully scraped data for {}#{}", account.username, account.tagline),
                Err(e) => eprintln!("Error scraping data for {}#{}: {}", account.username, account.tagline, e),
            }
        }
    }
}

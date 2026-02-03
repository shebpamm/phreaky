use libsql::{Builder, Connection};
use color_eyre::eyre::Result;

use crate::config::Config;


pub async fn get_db() -> Result<Connection> {
    let config = Config::get();

    let db = Builder::new_remote(config.db.url.clone(), config.db.token.clone())
        .build()
        .await?;

    let conn = db.connect()?;

    Ok(conn)
}

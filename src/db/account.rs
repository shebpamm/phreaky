use chrono::{Utc, DateTime};
use color_eyre::eyre::Result;
use libsql::params;
use serde::{Serialize, Deserialize};

use crate::db::connection;
use crate::db::utils;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub id: i32,
    pub puuid: String,
    pub username: String,
    pub tagline: String,
    pub region: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

impl TryFrom<libsql::Row> for Account {
    type Error = color_eyre::eyre::Report;

    fn try_from(row: libsql::Row) -> Result<Self, Self::Error> {
        Ok(Account {
            id: row.get(0)?,
            puuid: row.get(1)?,
            username: row.get(2)?,
            tagline: row.get(3)?,
            region: row.get(4)?,
            created_at: utils::parse_date(&row.get::<String>(5)?)?,
            updated_at: utils::parse_date(&row.get::<String>(6)?)?,
        })
    }
}

pub async fn list_accounts() -> Result<Vec<Account>> {
    let conn = connection::get_db().await?;

    let mut rows = conn.query("SELECT id, puuid, username, tagline, region, created_at, updated_at FROM accounts", params![]).await?;

    let mut accounts = Vec::new();
    while let Some(row) = rows.next().await? {
        let account: Account = row.try_into()?;
        accounts.push(account);
    }

    Ok(accounts)
}

pub async fn get_account(puuid: &str) -> Result<Account> {
    let conn = connection::get_db().await?;

    let mut rows = conn.query(
        "SELECT id, puuid, username, tagline, region, created_at, updated_at FROM accounts WHERE puuid = ?",
        params![puuid],
    ).await?;

    if let Some(row) = rows.next().await? {
        let account: Account = row.try_into()?;
        Ok(account)
    } else {
        Err(color_eyre::eyre::eyre!("Account with Player UUID {} not found", puuid))
    }
}

pub async fn add_account(username: &str, tagline: &str, region: &str) -> Result<Account> {
    let conn = connection::get_db().await?;

    let account = crate::riot::get_player_info(username, tagline).await?;

    conn.execute(
        "INSERT INTO accounts (puuid, username, tagline, region) VALUES (?, ?, ?, ?)",
        params![account.puuid.clone(), account.username, account.tagline, region],
    ).await?;

    get_account(&account.puuid).await
}

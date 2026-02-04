use color_eyre::eyre::Context;
use thiserror::Error;
use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};

use crate::db::connection;
use crate::db::account;
use crate::db::errors::Result;
use crate::db::utils;


use crate::riot::PlayerStats;


#[derive(Error, Debug)]
pub enum StatError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] libsql::Error),

    #[error("Internal error: {0}")]
    InternalError(#[from] color_eyre::eyre::Report),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    pub account: account::Account,
    pub queue_type: String,
    pub tier: String,
    pub rank: String,
    pub league_points: u32,
    pub wins: u32,
    pub losses: u32,
    pub hot_streak: bool,
    pub created_at: DateTime<Utc>,
}

impl PartialEq<PlayerStats> for Stat {
    fn eq(&self, other: &PlayerStats) -> bool {
        self.queue_type == other.queue_type &&
        self.tier == other.tier &&
        self.rank == other.rank &&
        self.league_points == other.league_points &&
        self.wins == other.wins &&
        self.losses == other.losses &&
        self.hot_streak == other.hot_streak
    }
}

pub async fn insert_stats(stats: PlayerStats) -> Result<()> {
    let conn = connection::get_db().await?;

    let account = account::get_account(&stats.puuid).await.with_context(|| format!("Failed to get account while inserting stats for PUUID: {}", stats.puuid))?;

    conn.execute(
        "INSERT INTO stats (account_id, queue_type, tier, rank, league_points, wins, losses, hot_streak) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        libsql::params![
            account.id,
            stats.queue_type,
            stats.tier,
            stats.rank,
            stats.league_points,
            stats.wins,
            stats.losses,
            stats.hot_streak,
        ],
    ).await?;

    Ok(())
}

pub async fn add_if_changed(stats: PlayerStats) -> Result<bool> {
    let existing_stat = get_queue_stat(&stats.puuid, &stats.queue_type).await?;

    match existing_stat {
        Some(stat) => {
            if stat != stats {
                insert_stats(stats).await?;
                return Ok(true)
            }
        }
        None => {
            insert_stats(stats).await?;
            return Ok(true)
        }
    }

    Ok(false)
}

pub async fn get_queue_stat(puuid: &str, queue_type: &str) -> Result<Option<Stat>> {
    let conn = connection::get_db().await?;

    let account = account::get_account(puuid).await.with_context(|| format!("Failed to get account while fetching stat for PUUID: {}", puuid))?;

    let mut rows = conn.query(
        "SELECT queue_type, tier, rank, league_points, wins, losses, hot_streak, created_at FROM stats WHERE account_id = ? AND queue_type = ? ORDER BY created_at DESC LIMIT 1",
        libsql::params![account.id, queue_type],
    ).await?;

    if let Some(row) = rows.next().await? {
        let stat = Stat {
            account,
            queue_type: row.get(0)?,
            tier: row.get(1)?,
            rank: row.get(2)?,
            league_points: row.get(3)?,
            wins: row.get(4)?,
            losses: row.get(5)?,
            hot_streak: row.get(6)?,
            created_at: utils::parse_date(&row.get::<String>(7)?)?,
        };
        Ok(Some(stat))
    } else {
        Ok(None)
    }
}

pub async fn get_queue_stats(puuid: &str, queue_type: &str) -> Result<Vec<Stat>> {
    let conn = connection::get_db().await?;

    let account = account::get_account(puuid).await.with_context(|| format!("Failed to get account while fetching stats for PUUID: {}", puuid))?;

    let mut rows = conn.query(
        "SELECT queue_type, tier, rank, league_points, wins, losses, hot_streak FROM stats WHERE account_id = ? AND queue_type = ?",
        libsql::params![account.id, queue_type],
    ).await?;

    let mut stats = Vec::new();
    while let Some(row) = rows.next().await? {
        let stat = Stat {
            account: account.clone(),
            queue_type: row.get(0)?,
            tier: row.get(1)?,
            rank: row.get(2)?,
            league_points: row.get(3)?,
            wins: row.get(4)?,
            losses: row.get(5)?,
            hot_streak: row.get(6)?,
            created_at: utils::parse_date(&row.get::<String>(7)?)?,
        };
        stats.push(stat);
    }

    Ok(stats)
}

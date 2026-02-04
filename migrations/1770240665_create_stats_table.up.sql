/**
pub struct PlayerStats {
    queue_type: String,
    tier: String,
    rank: String,
    league_points: u32,
    wins: u32,
    losses: u32,
    hot_streak: bool,
}

The table holds the stats for each player,
and is linked to the accounts table via a foreign key.
Each time a player's stats are updated,
a new record is inserted into this table with the current timestamp.
This allows us to track the history of a player's stats over time.
*/

CREATE TABLE stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,

    queue_type TEXT NOT NULL,
    tier TEXT NOT NULL,
    rank TEXT NOT NULL,
    league_points INTEGER NOT NULL,
    wins INTEGER NOT NULL,
    losses INTEGER NOT NULL,
    hot_streak INTEGER NOT NULL,

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (account_id) REFERENCES accounts(id)
);

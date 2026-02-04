use thiserror::Error;

use crate::db::account::AccountError;
use crate::db::stats::StatError;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Account error: {0}")]
    AccountError(#[from] AccountError),
    #[error("Stat error: {0}")]
    StatError(#[from] StatError),
    #[error("Database error: {0}")]
    DatabaseError(#[from] libsql::Error),
    #[error("Internal error: {0}")]
    InternalError(#[from] color_eyre::eyre::Report),
}

pub type Result<T> = std::result::Result<T, DbError>;

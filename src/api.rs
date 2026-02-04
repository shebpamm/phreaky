use axum::{Router, http::StatusCode, response::IntoResponse};
use color_eyre::eyre::ErrReport;
use crate::db::account::AccountError;

pub mod account;

#[derive(Debug)]
pub enum ApiError {
    Internal(ErrReport),
    Account(AccountError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("API Error: {:?}", self);

        match self {
            ApiError::Account(account_error) => match account_error {
                AccountError::NotFound(msg) => (
                    StatusCode::NOT_FOUND,
                    format!("Not Found: {}", msg),
                ).into_response(),
                AccountError::AlreadyExists(msg) => (
                    StatusCode::CONFLICT,
                    format!("Account already exists: {}", msg),
                ).into_response(),
                AccountError::InternalError(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Internal Server Error: {}", err),
                ).into_response(),
                AccountError::DatabaseError(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database Error: {}", err),
                ).into_response(),
            },
            ApiError::Internal(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Server Error: {}", err),
            ).into_response(),
    }
    }
}

impl From<ErrReport> for ApiError {
    fn from(err: ErrReport) -> Self {
        ApiError::Internal(err)
    }
}

impl From<AccountError> for ApiError {
    fn from(err: AccountError) -> Self {
        ApiError::Account(err)
    }
}

type ApiResult<T> = std::result::Result<T, ApiError>;

pub fn get_routes() -> Router {
    let app = Router::new()
        .nest("/account", account::get_routes());
    app
}

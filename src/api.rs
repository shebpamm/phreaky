use axum::{Router, http::StatusCode, response::IntoResponse};
use color_eyre::eyre::{Result, ErrReport};

pub mod account;

#[derive(Debug)]
pub struct ApiError(pub ErrReport); 

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("API Error: {:?}", self);

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", self.0),
        ).into_response()
    }
}

impl From<ErrReport> for ApiError {
    fn from(err: ErrReport) -> Self {
        ApiError(err)
    }
}

type ApiResult<T> = Result<T, ApiError>;

pub fn get_routes() -> Router {
    let app = Router::new()
        .nest("/account", account::get_routes());
    app
}

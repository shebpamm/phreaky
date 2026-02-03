use axum::Router;
use axum::routing::get;

use crate::db;

async fn list_accounts() -> &'static str {
    "List of accounts"
}

pub fn get_routes() -> Router {
    let app = Router::new()
        .route("/", get(list_accounts));
    app
}

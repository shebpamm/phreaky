use axum::Router;
use axum::routing::get;
use axum::Json;

use crate::db::account;
use crate::api::ApiResult;

async fn list_accounts() -> ApiResult<Json<Vec<account::Account>>> {
    let accounts = account::list_accounts().await?;

    Ok(Json(accounts))
}

pub fn get_routes() -> Router {
    let app = Router::new()
        .route("/", get(list_accounts));
    app
}

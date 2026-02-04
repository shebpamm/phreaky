use axum::Router;
use axum::routing::get;
use axum::Json;
use axum::extract::Path;

use crate::db::account;
use crate::api::ApiResult;

async fn list_accounts() -> ApiResult<Json<Vec<account::Account>>> {
    let accounts = account::list_accounts().await?;

    Ok(Json(accounts))
}

async fn get_account(Path(puuid): Path<String>) -> ApiResult<Json<account::Account>> {
    let account = account::get_account(&puuid).await?;

    Ok(Json(account))
}

pub fn get_routes() -> Router {
    let app = Router::new()
        .route("/", get(list_accounts))
        .route("/{*puuid}", get(get_account));
    app
}

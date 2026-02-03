use axum::Router;

pub mod account;

pub fn get_routes() -> Router {
    let app = Router::new()
        .nest("/account", account::get_routes());
    app
}

use tokio::runtime::Builder;
use color_eyre::eyre::Result;
use axum::Router;

use crate::config::Config;
use crate::api;

fn create_router() -> Router {
    use axum::{routing::get, Router};

    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api", api::get_routes());

    router
}

pub fn start_runtime() -> Result<()> {
    let rt = Builder::new_multi_thread()
        .enable_all()
        .build()?;

    rt.block_on(serve())
}

async fn serve() ->  Result<()> {
    let config = Config::get();

    let (close_tx, close_rx) = tokio::sync::oneshot::channel::<()>();

    let router = create_router();

    let server_handle = tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind(config.server.address()).await.unwrap();
            axum::serve(listener, router)
                .with_graceful_shutdown(async {
                    _ = close_rx.await;
                })
                .await
                .unwrap();
    });

    let worker_handle = tokio::spawn(async move {
        crate::worker::background_worker().await.unwrap();
    });

    println!("Server running at http://{}", config.server.address());

    tokio::select! {
        _ = server_handle => {
            println!("Server has stopped.");
        }

        _ = worker_handle => {
            println!("Background worker has stopped. Shutting down axum server.");
            close_tx.send(()).ok();
        }

        _ = tokio::signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down.");
            close_tx.send(()).ok();
        }
    }

    Ok(())
}

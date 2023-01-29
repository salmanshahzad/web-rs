use std::{net::SocketAddr, sync::Arc};

use axum::{middleware, Router, Server};
use axum_extra::routing::SpaRouter;
use tokio::signal;

use crate::state::AppState;

mod env;
mod error;
mod routes;
mod state;
mod utils;

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState::new().await);
    let port = app_state.env().port();

    let api_router = Router::new()
        .nest("/health", routes::health::router())
        .nest("/session", routes::session::router())
        .nest("/user", routes::user::router(Arc::clone(&app_state)));
    let app = Router::new()
        .nest("/api", api_router)
        .merge(SpaRouter::new("/", "public").index_file("index.html"))
        .layer(middleware::from_fn(utils::cors::cors))
        .with_state(Arc::clone(&app_state));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let server = Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            signal::ctrl_c().await.ok();
        });

    match server.await {
        Ok(()) => shutdown(app_state).await,
        Err(err) => eprintln!("Server error: {err}"),
    }
}

async fn shutdown(state: Arc<AppState>) {
    println!("Shutting down server");
    state.db().close().await;
}

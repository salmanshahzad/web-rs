use std::{net::SocketAddr, sync::Arc, time::Duration};

use async_redis_session::RedisSessionStore;
use axum::{middleware, Router, Server};
use axum_extra::routing::SpaRouter;
use axum_sessions::SessionLayer;
use tokio::signal;

use crate::state::AppState;

mod config;
mod error;
mod routes;
mod state;
mod utils;

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState::new().await);
    let config = app_state.config();

    let redis_session_store =
        RedisSessionStore::new(config.redis_url()).expect("Could not create Redis session store");
    let session_layer = SessionLayer::new(redis_session_store, config.cookie_secret().as_bytes())
        .with_session_ttl(Some(Duration::from_secs(7 * 24 * 60 * 60)));

    let api_router = Router::new()
        .nest("/health", routes::health::router())
        .nest("/session", routes::session::router())
        .nest("/user", routes::user::router(Arc::clone(&app_state)));
    let app = Router::new()
        .nest("/api", api_router)
        .merge(SpaRouter::new("/", "public"))
        .layer(session_layer)
        .layer(middleware::from_fn(utils::cors::cors))
        .with_state(Arc::clone(&app_state));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port()));
    let server = Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            signal::ctrl_c().await.ok();
        });

    println!("Server listening on port {}", config.port());
    match server.await {
        Ok(()) => shutdown(app_state).await,
        Err(err) => eprintln!("Server error: {err}"),
    }
}

async fn shutdown(state: Arc<AppState>) {
    println!("Shutting down server");
    state.db().close().await;
}

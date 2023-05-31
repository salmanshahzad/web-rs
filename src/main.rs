use std::{net::SocketAddr, path::Path, sync::Arc, time::Duration};

use async_redis_session::RedisSessionStore;
use axum::{Router, Server};
use axum_sessions::SessionLayer;
use log::{error, info};
use sqlx::migrate::Migrator;
use tokio::signal;
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::state::AppState;

mod config;
mod error;
mod routes;
mod state;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();

    let app_state = Arc::new(AppState::new().await);
    let config = app_state.config();
    info!("Initialized app state");

    let migrator = Migrator::new(Path::new("./migrations"))
        .await
        .expect("Could not create migrator");
    migrator
        .run(app_state.db())
        .await
        .expect("Could not run migrations");

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
        .nest_service("/", ServeDir::new("public"))
        .layer(session_layer)
        .layer(CorsLayer::very_permissive())
        .with_state(Arc::clone(&app_state));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port()));
    let server = Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            signal::ctrl_c().await.ok();
        });

    info!("Server listening on port {}", config.port());
    match server.await {
        Ok(()) => shutdown(app_state).await,
        Err(err) => error!("Server error: {err}"),
    }
}

async fn shutdown(state: Arc<AppState>) {
    info!("Shutting down server");
    state.db().close().await;
}

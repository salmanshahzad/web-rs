use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(health))
}

async fn health() -> Response {
    StatusCode::NO_CONTENT.into_response()
}

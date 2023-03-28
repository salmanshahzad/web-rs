use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, post},
    Router,
};
use axum_sessions::extractors::WritableSession;
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::{make_error, ResponseResult},
    state::AppState,
    utils::json::Json,
};

#[derive(Deserialize, Validate)]
struct SignIn {
    #[validate(length(min = 1, message = "Username is required"))]
    username: String,
    #[validate(length(min = 1, message = "Password is required"))]
    password: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(sign_in))
        .route("/", delete(sign_out))
}

async fn sign_in(
    State(state): State<Arc<AppState>>,
    mut session: WritableSession,
    Json(payload): Json<SignIn>,
) -> ResponseResult {
    let user = sqlx::query!(
        "SELECT * FROM \"user\" WHERE username = $1;",
        payload.username
    )
    .fetch_optional(state.db())
    .await?;

    match user {
        Some(user) => {
            let verify_password =
                argon2::verify_encoded(&user.password, payload.password.as_bytes())?;
            if verify_password {
                session.insert("user_id", user.id)?;
                Ok(StatusCode::NO_CONTENT.into_response())
            } else {
                Ok(make_error(
                    StatusCode::UNAUTHORIZED,
                    "Incorrect username or password",
                ))
            }
        }
        None => Ok(make_error(
            StatusCode::UNAUTHORIZED,
            "Incorrect username or password",
        )),
    }
}

async fn sign_out(mut session: WritableSession) -> ResponseResult {
    session.destroy();
    Ok(StatusCode::NO_CONTENT.into_response())
}

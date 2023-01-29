use std::sync::Arc;

use axum::{
    extract::State,
    handler::Handler,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, put},
    Extension, Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    error::{make_error, ResponseResult},
    utils::auth,
    AppState,
};

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    let m = middleware::from_fn_with_state(state, auth::verify_cookie);
    Router::new()
        .route(
            "/",
            get(get_user.layer(m.clone()))
                .post(create_user)
                .delete(delete_user.layer(m.clone())),
        )
        .route("/username", put(change_username.layer(m.clone())))
        .route("/password", put(change_password.layer(m.clone())))
}

async fn get_user(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> ResponseResult {
    let user = sqlx::query!("SELECT username FROM \"user\" WHERE id = $1;", user_id)
        .fetch_one(state.db())
        .await?;
    let body = json!({
        "username": user.username,
    })
    .to_string();
    Ok(body.into_response())
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
    password: String,
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> ResponseResult {
    let username_exists = sqlx::query!(
        "SELECT COUNT(*) FROM \"user\" WHERE username = $1;",
        payload.username
    )
    .fetch_one(state.db())
    .await?;

    match username_exists.count {
        Some(1) => Ok(make_error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Username already exists",
        )),
        Some(0) => {
            let hashed_password = argon2::hash_encoded(
                payload.password.as_bytes(),
                state.env().password_salt().as_bytes(),
                &Default::default(),
            )?;
            sqlx::query!(
                "INSERT INTO \"user\" (username, password) VALUES ($1, $2)",
                payload.username,
                hashed_password
            )
            .execute(state.db())
            .await?;
            Ok(StatusCode::CREATED.into_response())
        }
        _ => unreachable!(),
    }
}

#[derive(Deserialize)]
pub struct ChangeUsername {
    username: String,
}

async fn change_username(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
    Json(payload): Json<ChangeUsername>,
) -> ResponseResult {
    let username_exists = sqlx::query!(
        "SELECT COUNT(*) FROM \"user\" WHERE id != $1 AND username = $2;",
        user_id,
        payload.username
    )
    .fetch_one(state.db())
    .await?;
    match username_exists.count {
        Some(0) => {
            sqlx::query!(
                "UPDATE \"user\" SET username = $1 WHERE id = $2;",
                payload.username,
                user_id
            )
            .execute(state.db())
            .await?;
            Ok(StatusCode::NO_CONTENT.into_response())
        }
        Some(1) => Ok(make_error(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Username already exists",
        )),
        _ => unreachable!(),
    }
}

#[derive(Deserialize)]
pub struct ChangePassword {
    new_password: String,
}

async fn change_password(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
    Json(payload): Json<ChangePassword>,
) -> ResponseResult {
    let hashed_password = argon2::hash_encoded(
        payload.new_password.as_bytes(),
        state.env().password_salt().as_bytes(),
        &Default::default(),
    )?;
    sqlx::query!(
        "UPDATE \"user\" SET password = $1 WHERE id = $2;",
        hashed_password,
        user_id
    )
    .execute(state.db())
    .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

async fn delete_user(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<i32>,
) -> ResponseResult {
    sqlx::query!("DELETE FROM \"user\" WHERE id = $1;", user_id)
        .execute(state.db())
        .await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}

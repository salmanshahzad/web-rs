use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use cookie::{
    time::{Duration, OffsetDateTime},
    Cookie,
};
use serde::Deserialize;

use crate::{
    error::{make_error, ResponseResult},
    state::AppState,
    utils::token,
};

#[derive(Deserialize)]
struct SignIn {
    username: String,
    password: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", post(sign_in).delete(sign_out))
}

async fn sign_in(
    State(state): State<Arc<AppState>>,
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
                let mut expires = OffsetDateTime::now_utc();
                expires += Duration::weeks(1);
                let token = token::encode(
                    user.id,
                    expires.unix_timestamp(),
                    state.config().jwt_secret(),
                )?;

                let cookie = Cookie::build("token", &token)
                    .expires(expires)
                    .http_only(true)
                    .finish();

                let headers = HeaderMap::from_iter([(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&cookie.to_string())?,
                )]);
                Ok((StatusCode::OK, headers).into_response())
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

async fn sign_out() -> ResponseResult {
    let mut cookie = Cookie::new("token", "");
    cookie.make_removal();

    let headers = HeaderMap::from_iter([(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string())?,
    )]);
    Ok((StatusCode::NO_CONTENT, headers).into_response())
}

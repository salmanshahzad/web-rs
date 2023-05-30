use argon2::Error as Argon2Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::error;
use redis::RedisError;
use serde_json::{json, Error as SerdeJsonError};
use sqlx::Error as DbError;
use thiserror::Error;

pub type ResponseResult = Result<Response, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("")]
    Argon2Error(#[from] Argon2Error),
    #[error("")]
    DbError(#[from] DbError),
    #[error("")]
    RedisError(#[from] RedisError),
    #[error("")]
    SerdeJsonError(#[from] SerdeJsonError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        error!("{self:?}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

pub fn make_error(status: StatusCode, message: &str) -> Response {
    let body = json!({
        "message": message,
        "status": status.as_u16(),
    })
    .to_string();
    (status, body).into_response()
}

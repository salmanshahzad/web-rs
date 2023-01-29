use argon2::Error as Argon2Error;
use axum::{
    http::{header::InvalidHeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use jsonwebtoken::errors::Error as JwtError;
use redis::RedisError;
use serde_json::json;
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
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error("")]
    JwtError(#[from] JwtError),
    #[error("")]
    RedisError(#[from] RedisError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        eprintln!("ERROR: {self:?}");
        let status = match self {
            Self::JwtError(_) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        status.into_response()
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

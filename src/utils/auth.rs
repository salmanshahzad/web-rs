use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, HeaderMap, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;

use crate::{error::ResponseResult, state::AppState, utils::token};

pub async fn verify_cookie<B>(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    mut req: Request<B>,
    next: Next<B>,
) -> ResponseResult {
    let cookie = jar.get("token");
    match cookie {
        Some(cookie) => {
            let user_id = token::decode(cookie.value(), state.config().jwt_secret())?;
            let user = sqlx::query!("SELECT COUNT(*) FROM \"user\" WHERE id = $1;", user_id)
                .fetch_one(state.db())
                .await?;
            match user.count {
                Some(1) => {
                    req.extensions_mut().insert(user_id);
                    Ok(next.run(req).await)
                }
                Some(0) => {
                    eprintln!("User with id {user_id} does not exist but was given in cookie");
                    Ok(StatusCode::UNAUTHORIZED.into_response())
                }
                _ => unreachable!(),
            }
        }
        None => Ok(StatusCode::UNAUTHORIZED.into_response()),
    }
}

pub async fn verify_token<B>(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut req: Request<B>,
    next: Next<B>,
) -> ResponseResult {
    let auth = headers.get(header::AUTHORIZATION);
    match auth {
        Some(auth) => {
            let auth = auth.to_str().unwrap_or_default();
            if auth.starts_with("Bearer ") {
                let token = auth.split(' ').nth(1).unwrap_or_default();
                let user_id = token::decode(token, state.config().jwt_secret())?;
                let user = sqlx::query!("SELECT COUNT(*) FROM \"user\" WHERE id = $1;", user_id)
                    .fetch_one(state.db())
                    .await?;
                match user.count {
                    Some(1) => {
                        req.extensions_mut().insert(user_id);
                        Ok(next.run(req).await)
                    }
                    Some(0) => {
                        eprintln!("User with id {user_id} does not exist but was given in token");
                        Ok(StatusCode::UNAUTHORIZED.into_response())
                    }
                    _ => unreachable!(),
                }
            } else {
                Ok(StatusCode::UNAUTHORIZED.into_response())
            }
        }
        None => Ok(StatusCode::UNAUTHORIZED.into_response()),
    }
}

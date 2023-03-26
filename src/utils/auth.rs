use std::sync::Arc;

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use axum_sessions::extractors::WritableSession;

use crate::{error::ResponseResult, state::AppState};

#[derive(Clone, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

pub async fn verify_cookie<B>(
    State(state): State<Arc<AppState>>,
    mut session: WritableSession,
    mut req: Request<B>,
    next: Next<B>,
) -> ResponseResult {
    let user_id = session.get::<i32>("user_id");
    match user_id {
        Some(user_id) => {
            let user = sqlx::query_as!(User, "SELECT * FROM \"user\" WHERE id = $1;", user_id)
                .fetch_optional(state.db())
                .await?;
            match user {
                Some(user) => {
                    req.extensions_mut().insert(user);
                    session.regenerate();
                    Ok(next.run(req).await)
                }
                None => Ok(StatusCode::UNAUTHORIZED.into_response()),
            }
        }
        None => Ok(StatusCode::UNAUTHORIZED.into_response()),
    }
}

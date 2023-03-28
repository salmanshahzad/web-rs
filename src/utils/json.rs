use std::{borrow::Cow, collections::HashMap};

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest},
    http::{Request, StatusCode},
};
use serde_json::json;
use validator::Validate;

pub struct Json<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for Json<T>
where
    axum::Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    S: Send + Sync,
    B: Send + 'static,
    T: Validate + 'static,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => match value.validate() {
                Ok(()) => Ok(Self(value.0)),
                Err(err) => {
                    let errors = HashMap::<&str, Vec<Cow<str>>>::from_iter(
                        err.field_errors().iter().filter_map(|(field, errs)| {
                            let messages = errs
                                .iter()
                                .filter_map(|err| err.message.clone())
                                .collect::<Vec<_>>();
                            if messages.is_empty() {
                                None
                            } else {
                                Some((*field, messages))
                            }
                        }),
                    );
                    let payload = json!({
                        "errors": errors,
                    })
                    .to_string();
                    Err((StatusCode::UNPROCESSABLE_ENTITY, payload))
                }
            },
            Err(rejection) => {
                let payload = json!({
                    "message": rejection.body_text(),
                    "status": rejection.status().as_u16(),
                })
                .to_string();
                Err((rejection.status(), payload))
            }
        }
    }
}

use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    exp: i64,
    user_id: i32,
}

pub fn encode(user_id: i32, exp: i64, secret: &str) -> Result<String, Error> {
    let header = Header::default();
    let claims = Claims { exp, user_id };
    let key = EncodingKey::from_secret(secret.as_bytes());
    jsonwebtoken::encode(&header, &claims, &key)
}

pub fn decode(token: &str, secret: &str) -> Result<i32, Error> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::default();
    let claims = jsonwebtoken::decode::<Claims>(token, &key, &validation)?;
    Ok(claims.claims.user_id)
}

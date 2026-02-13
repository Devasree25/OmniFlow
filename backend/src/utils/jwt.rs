use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;
use uuid::Uuid;

use crate::models::{Claims, User, UserRole};

pub fn create_jwt(user: &User, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration_hours: i64 = env::var("JWT_EXPIRATION_HOURS")
        .unwrap_or_else(|_| "24".to_string())
        .parse()
        .unwrap_or(24);

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(expiration_hours))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id,
        tenant_id: user.tenant_id,
        email: user.email.clone(),
        role: user.role.clone(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

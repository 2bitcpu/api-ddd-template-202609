use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use common::BoxError;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct Claims {
    sub: String, // subject (ユーザーの識別子)
    iss: String, // issuer (JWTの発行者)
    iat: i64,    // Issued At (発行日時)
    exp: i64,    // expiration time (トークンの有効期限)
    jti: String, // JWT ID (JWTの一意な識別子)
}

pub fn generate(
    subject: String,
    issure: String,
    expiration_minutes: i64,
    jwt_id: String,
    secret: String,
) -> Result<String, BoxError> {
    let now = Utc::now();
    let expiration = now
        .checked_add_signed(chrono::Duration::minutes(expiration_minutes))
        .ok_or("json web token timestamp overflow")?
        .timestamp();

    let claims = Claims {
        sub: subject,
        iss: issure,
        iat: now.timestamp(),
        exp: expiration,
        jti: jwt_id,
    };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

pub fn verify(token: String, issure: String, secret: String) -> Result<(String, String), BoxError> {
    let mut validation = Validation::default();
    validation.validate_exp = true;
    validation.leeway = 30;
    validation.set_issuer(&[issure]);

    let claims = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?
    .claims;

    Ok((claims.sub, claims.jti))
}

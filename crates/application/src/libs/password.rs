use argon2::{
    Argon2, password_hash,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

use common::BoxError;

pub async fn hash(plain: String) -> Result<String, BoxError> {
    tokio::task::spawn_blocking(move || {
        Ok(Argon2::default()
            .hash_password(plain.as_bytes())?
            .to_string())
    })
    .await?
}

pub async fn verify(plain: String, hash: String) -> Result<bool, BoxError> {
    tokio::task::spawn_blocking(move || {
        let hash = PasswordHash::new(&hash)?;
        match Argon2::default().verify_password(plain.as_bytes(), &hash) {
            Ok(()) => Ok(true),
            Err(password_hash::Error::PasswordInvalid) => Ok(false),
            Err(e) => Err(e.into()),
        }
    })
    .await?
}

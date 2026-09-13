use axum::{
    RequestExt,
    extract::{FromRequestParts, Request, State},
    http::{StatusCode, request::Parts},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub account: String,
    #[allow(unused)]
    pub email: Option<String>,
}

use application::Applications;

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Self>()
            .cloned()
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}

pub async fn auth_guard(
    State(module): State<Arc<dyn Applications>>,
    mut request: Request,
    next: Next,
) -> axum::response::Result<Response> {
    let bearer = request
        .extract_parts::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let token = bearer.token();

    let (account, email) = module
        .auth()
        .authenticate(token.to_string())
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(AuthUser { account, email });

    Ok(next.run(request).await)
}

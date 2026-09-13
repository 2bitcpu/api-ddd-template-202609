use axum::extract::State;
use axum::{Json, http::StatusCode, response::IntoResponse};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use std::sync::Arc;

use crate::error::ApiError;
use application::{
    Applications,
    model::auth::{SigninRequestDto, SignupRequestDto},
};

pub async fn signup(
    State(applications): State<Arc<dyn Applications>>,
    Json(dto): Json<SignupRequestDto>,
) -> Result<impl IntoResponse, ApiError> {
    applications.auth().signup(dto).await?;
    Ok(StatusCode::CREATED.into_response())
}

pub async fn signin(
    State(usecases): State<Arc<dyn Applications>>,
    Json(dto): Json<SigninRequestDto>,
) -> Result<impl IntoResponse, ApiError> {
    let res = usecases.auth().signin(dto).await?;
    Ok(Json(res).into_response())
}

pub async fn signout(
    State(applications): State<Arc<dyn Applications>>,
    bearer: Option<TypedHeader<Authorization<Bearer>>>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("signout bearer: {:?}", bearer);

    if let Some(bearer) = bearer {
        applications
            .auth()
            .signout(bearer.token().to_string())
            .await?;
    }
    Ok(StatusCode::OK.into_response())
}

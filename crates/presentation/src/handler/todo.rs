use crate::error::ApiError;
use crate::middleware::auth::AuthUser;

use application::{
    Applications,
    model::todo::{TodoEntryRequestDto, TodoReplacceRequestDto},
};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

pub async fn entry(
    State(applications): State<Arc<dyn Applications>>,
    Extension(auth): Extension<AuthUser>,
    Json(dto): Json<TodoEntryRequestDto>,
) -> Result<impl IntoResponse, ApiError> {
    let res = applications.todo().entry(dto, auth.account).await?;
    Ok((StatusCode::CREATED, Json(res)).into_response())
}

pub async fn replace(
    State(applications): State<Arc<dyn Applications>>,
    Extension(auth): Extension<AuthUser>,
    Json(dto): Json<TodoReplacceRequestDto>,
) -> Result<impl IntoResponse, ApiError> {
    let res = applications.todo().replace(dto, auth.account).await?;
    Ok(Json(res).into_response())
}

pub async fn remove(
    State(applications): State<Arc<dyn Applications>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    applications.todo().remove(id, auth.account).await?;
    Ok(StatusCode::OK.into_response())
}

pub async fn list(
    State(applications): State<Arc<dyn Applications>>,
    Extension(auth): Extension<AuthUser>,
) -> Result<impl IntoResponse, ApiError> {
    let res = applications.todo().list(auth.account).await?;
    Ok(Json(res).into_response())
}

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

pub async fn create(
    State(applications): State<Arc<dyn Applications>>,
    Extension(auth): Extension<AuthUser>,
    Json(dto): Json<TodoEntryRequestDto>,
) -> Result<impl IntoResponse, ApiError> {
    let res = applications.todo().create(dto, auth.account).await?;
    Ok((StatusCode::CREATED, Json(res)).into_response())
}

pub async fn update(
    State(applications): State<Arc<dyn Applications>>,
    Extension(auth): Extension<AuthUser>,
    Json(dto): Json<TodoReplacceRequestDto>,
) -> Result<impl IntoResponse, ApiError> {
    let res = applications.todo().update(dto, auth.account).await?;
    Ok(Json(res).into_response())
}

pub async fn read(
    State(applications): State<Arc<dyn Applications>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let res = applications.todo().read(id, auth.account).await?;
    Ok(Json(res).into_response())
}

pub async fn delete(
    State(applications): State<Arc<dyn Applications>>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    applications.todo().delete(id, auth.account).await?;
    Ok(StatusCode::OK.into_response())
}

pub async fn list(
    State(applications): State<Arc<dyn Applications>>,
    Extension(auth): Extension<AuthUser>,
) -> Result<impl IntoResponse, ApiError> {
    let res = applications.todo().list(auth.account).await?;
    Ok(Json(res).into_response())
}

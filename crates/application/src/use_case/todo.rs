use std::sync::Arc;

use crate::error::AppError;
use crate::model::todo::{TodoEntryRequestDto, TodoReplacceRequestDto, TodoResponseDto};
use domain::Repositories;

pub struct TodoUseCase {
    repositories: Arc<dyn Repositories>,
}

impl TodoUseCase {
    pub fn new(repositories: Arc<dyn Repositories>) -> Self {
        Self { repositories }
    }

    pub async fn entry(
        &self,
        dto: TodoEntryRequestDto,
        account: String,
    ) -> Result<TodoResponseDto, AppError> {
        let todo = self
            .repositories
            .todo()
            .save(dto.to_model(account))
            .await
            .map_err(AppError::from)?;

        Ok(TodoResponseDto::from(todo))
    }

    pub async fn replace(
        &self,
        dto: TodoReplacceRequestDto,
        account: String,
    ) -> Result<TodoResponseDto, AppError> {
        let todo = self
            .repositories
            .todo()
            .find(&dto.id)
            .await?
            .ok_or_else(|| AppError::DataNotFound(format!("todo not found: {:#?}", dto.id)))?;

        if todo.owner != account {
            return Err(AppError::Forbidden("Permission denied.".to_string()));
        }

        let todo = self
            .repositories
            .todo()
            .replace(dto.to_model(account))
            .await
            .map_err(AppError::from)?;

        Ok(TodoResponseDto::from(todo))
    }

    pub async fn remove(&self, id: String, account: String) -> Result<(), AppError> {
        let todo = self
            .repositories
            .todo()
            .find(&id)
            .await?
            .ok_or_else(|| AppError::DataNotFound(format!("todo not found: {:#?}", id)))?;

        if todo.owner != account {
            return Err(AppError::Forbidden("Permission denied.".to_string()));
        }

        self.repositories
            .todo()
            .remove(&id)
            .await
            .map_err(AppError::from)
    }

    pub async fn list(&self, account: String) -> Result<Vec<TodoResponseDto>, AppError> {
        let todos = self
            .repositories
            .todo()
            .list(&account)
            .await?
            .into_iter()
            .map(TodoResponseDto::from)
            .collect();

        Ok(todos)
    }
}

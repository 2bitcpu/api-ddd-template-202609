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

    pub async fn create(
        &self,
        dto: TodoEntryRequestDto,
        owner: String,
    ) -> Result<TodoResponseDto, AppError> {
        let todo = self
            .repositories
            .todo()
            .create(dto.to_model(owner))
            .await
            .map_err(AppError::from)?;

        Ok(TodoResponseDto::from(todo))
    }

    pub async fn update(
        &self,
        dto: TodoReplacceRequestDto,
        owner: String,
    ) -> Result<TodoResponseDto, AppError> {
        let todo = self
            .repositories
            .todo()
            .read(&dto.id)
            .await?
            .ok_or_else(|| AppError::DataNotFound(format!("todo not found: {:#?}", dto.id)))?;

        if todo.owner != owner {
            return Err(AppError::Forbidden("Permission denied.".to_string()));
        }

        let todo = self
            .repositories
            .todo()
            .update(dto.to_model(owner))
            .await
            .map_err(AppError::from)?;

        Ok(TodoResponseDto::from(todo))
    }

    pub async fn read(
        &self,
        id: String,
        owner: String,
    ) -> Result<Option<TodoResponseDto>, AppError> {
        let todo = self.repositories.todo().read(&id).await?;

        if let Some(t) = &todo {
            if t.owner != owner {
                return Err(AppError::Forbidden("Permission denied.".to_string()));
            }
        }

        Ok(todo.map(TodoResponseDto::from))
    }

    pub async fn delete(&self, id: String, account: String) -> Result<(), AppError> {
        let todo = self
            .repositories
            .todo()
            .read(&id)
            .await?
            .ok_or_else(|| AppError::DataNotFound(format!("todo not found: {:#?}", id)))?;

        if todo.owner != account {
            return Err(AppError::Forbidden("Permission denied.".to_string()));
        }

        self.repositories
            .todo()
            .delete(&id)
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

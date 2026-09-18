use async_trait::async_trait;

use crate::error::DomainError;
use crate::model::TodoModel;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&self, model: TodoModel) -> Result<TodoModel, DomainError>;
    async fn update(&self, model: TodoModel) -> Result<TodoModel, DomainError>;
    async fn read(&self, key: &str) -> Result<Option<TodoModel>, DomainError>;
    async fn delete(&self, id: &str) -> Result<(), DomainError>;
    async fn list(&self, owner: &str) -> Result<Vec<TodoModel>, DomainError>;
}

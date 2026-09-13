use async_trait::async_trait;

use crate::error::DomainError;
use crate::model::TodoModel;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn save(&self, todo: TodoModel) -> Result<TodoModel, DomainError>;
    async fn replace(&self, todo: TodoModel) -> Result<TodoModel, DomainError>;
    async fn find(&self, id: &str) -> Result<Option<TodoModel>, DomainError>;
    async fn remove(&self, id: &str) -> Result<(), DomainError>;
    async fn list(&self, owner: &str) -> Result<Vec<TodoModel>, DomainError>;
    async fn commit(&self) -> Result<(), DomainError>;
}

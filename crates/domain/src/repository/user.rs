use async_trait::async_trait;

use crate::error::DomainError;
use crate::model::UserModel;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, model: UserModel) -> Result<UserModel, DomainError>;
    async fn update(&self, model: UserModel) -> Result<UserModel, DomainError>;
    async fn read(&self, key: &str) -> Result<Option<UserModel>, DomainError>;
    async fn delete(&self, key: &str) -> Result<(), DomainError>;
}

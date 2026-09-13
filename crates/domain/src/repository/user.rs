use async_trait::async_trait;

use crate::error::DomainError;
use crate::model::UserModel;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: UserModel) -> Result<UserModel, DomainError>;
    async fn replace(&self, user: UserModel) -> Result<UserModel, DomainError>;
    async fn find(&self, account: &str) -> Result<Option<UserModel>, DomainError>;
    async fn remove(&self, account: &str) -> Result<(), DomainError>;
    async fn commit(&self) -> Result<(), DomainError>;
}

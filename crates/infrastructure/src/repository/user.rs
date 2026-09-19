use async_trait::async_trait;
use fjall::Keyspace;

use domain::{DomainError, UserRepository, model::UserModel};

use super::wrapper::{run_blocking, unexpected};

pub struct UserRepositoryImpl {
    keyspace: Keyspace,
}

impl UserRepositoryImpl {
    pub fn new(keyspace: Keyspace) -> Self {
        Self { keyspace }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, model: UserModel) -> Result<UserModel, DomainError> {
        let keyspace = self.keyspace.clone();

        run_blocking(move || {
            let key = model.account.clone();
            let value = serde_json::to_vec(&model)?;

            if keyspace.contains_key(&key).map_err(unexpected)? {
                return Err(DomainError::Conflict(format!("todo already exists: {key:?}")).into());
            }

            keyspace.insert(key, value).map_err(unexpected)?;

            Ok(model)
        })
        .await
    }

    async fn read(&self, key: &str) -> Result<Option<UserModel>, DomainError> {
        let keyspace = self.keyspace.clone();
        let key = key.to_owned();

        run_blocking(move || match keyspace.get(key).map_err(unexpected)? {
            Some(s) => Ok(Some(serde_json::from_slice(&s)?)),
            None => Ok(None),
        })
        .await
    }

    async fn update(&self, model: UserModel) -> Result<UserModel, DomainError> {
        let keyspace = self.keyspace.clone();

        run_blocking(move || {
            let key = model.account.clone();
            let value = serde_json::to_vec(&model)?;

            if !keyspace.contains_key(&key).map_err(unexpected)? {
                return Err(DomainError::NotFound(format!("todo not found: {key:?}")).into());
            }

            keyspace.insert(key, value).map_err(unexpected)?;

            Ok(model)
        })
        .await
    }

    async fn delete(&self, key: &str) -> Result<(), DomainError> {
        let keyspace = self.keyspace.clone();
        let key = key.to_owned();

        run_blocking(move || {
            if !keyspace.contains_key(&key).map_err(unexpected)? {
                return Err(DomainError::NotFound(format!("todo not found: {:#?}", key)).into());
            }

            keyspace.remove(key).map_err(unexpected)?;

            Ok(())
        })
        .await
    }
}

use async_trait::async_trait;
use fjall::Keyspace;

use domain::{DomainError, TodoRepository, model::TodoModel};

use crate::repository::wrapper::run_blocking;

pub struct TodoRepositoryImpl {
    keyspace: Keyspace,
}

impl TodoRepositoryImpl {
    pub fn new(keyspace: Keyspace) -> Self {
        Self { keyspace }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn create(&self, model: TodoModel) -> Result<TodoModel, DomainError> {
        let keyspace = self.keyspace.clone();

        run_blocking(move || {
            let key = model.id.clone();
            let value = serde_json::to_vec(&model)?;

            if keyspace.contains_key(&key)? {
                return Err(DomainError::Conflict(format!("todo already exists: {key:?}")).into());
            }

            keyspace.insert(key, value)?;

            Ok(model)
        })
        .await
    }

    async fn read(&self, key: &str) -> Result<Option<TodoModel>, DomainError> {
        let keyspace = self.keyspace.clone();
        let key = key.to_owned();

        run_blocking(move || match keyspace.get(key)? {
            Some(s) => Ok(Some(serde_json::from_slice(&s)?)),
            None => Ok(None),
        })
        .await
    }

    async fn update(&self, model: TodoModel) -> Result<TodoModel, DomainError> {
        let keyspace = self.keyspace.clone();

        run_blocking(move || {
            let key = model.id.clone();
            let value = serde_json::to_vec(&model)?;

            if !keyspace.contains_key(&key)? {
                return Err(DomainError::NotFound(format!("todo not found: {key:?}")).into());
            }

            keyspace.insert(key, value)?;

            Ok(model)
        })
        .await
    }

    async fn delete(&self, key: &str) -> Result<(), DomainError> {
        let keyspace = self.keyspace.clone();
        let key = key.to_owned();

        run_blocking(move || {
            if !keyspace.contains_key(&key)? {
                return Err(DomainError::NotFound(format!("todo not found: {:#?}", key)).into());
            }

            keyspace.remove(key)?;

            Ok(())
        })
        .await
    }

    async fn list(&self, owner: &str) -> Result<Vec<TodoModel>, DomainError> {
        let keyspace = self.keyspace.clone();
        let owner = owner.to_owned();

        run_blocking(move || {
            let mut todos = Vec::new();

            for entry in keyspace.iter() {
                let (_, value) = entry.into_inner()?;

                let todo: TodoModel = serde_json::from_slice(&value)?;

                if todo.owner == owner {
                    todos.push(todo);
                }
            }

            todos.sort_by_key(|todo| std::cmp::Reverse(todo.due_date));

            Ok(todos)
        })
        .await
    }
}

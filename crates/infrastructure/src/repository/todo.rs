use async_trait::async_trait;
use std::{
    collections::{HashMap, hash_map::Entry},
    sync::Arc,
};
use tokio::{
    sync::{RwLock, mpsc},
    time::{Duration, timeout},
};

use common::config;
use domain::{DomainError, TodoRepository, model::TodoModel};

use crate::model::todo::TodoData;
use crate::repository::{constant::TODO_FILE_NAME, worker::Command};

pub struct TodoRepositoryImpl {
    data: Arc<RwLock<TodoData>>,
    tx: mpsc::Sender<Command>,
    lock_timeout: Duration,
}

fn load() -> TodoData {
    let path = config().storage.data_dir.join(TODO_FILE_NAME);

    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(_) => return TodoData::new(),
    };

    match serde_json::from_str::<HashMap<String, TodoModel>>(&text) {
        Ok(map) => TodoData { map, dirty: false },
        Err(_) => TodoData::new(),
    }
}

impl TodoRepositoryImpl {
    pub fn new(tx: mpsc::Sender<Command>) -> Self {
        Self {
            data: Arc::new(RwLock::new(load())),
            tx,
            lock_timeout: Duration::from_millis(config().server.lock_timeout_millis),
        }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn save(&self, todo: TodoModel) -> Result<TodoModel, DomainError> {
        let mut data = timeout(self.lock_timeout, self.data.write())
            .await
            .map_err(|_| DomainError::Busy())?;

        match data.map.entry(todo.id.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(todo.clone());
                if !data.dirty {
                    data.dirty = true;
                    drop(data);
                    let _ = self.commit().await;
                }
                Ok(todo)
            }
            Entry::Occupied(_) => Err(DomainError::Conflict(format!(
                "todo already exists: {:#?}",
                todo.id
            ))),
        }
    }

    async fn find(&self, id: &str) -> Result<Option<TodoModel>, DomainError> {
        let data = self.data.read().await;
        Ok(data.map.get(id).cloned())
    }

    async fn replace(&self, todo: TodoModel) -> Result<TodoModel, DomainError> {
        let mut data = timeout(self.lock_timeout, self.data.write())
            .await
            .map_err(|_| DomainError::Busy())?;

        match data.map.entry(todo.id.clone()) {
            Entry::Vacant(_) => Err(DomainError::NotFound(format!(
                "todo not found: {:#?}",
                todo.id
            ))),
            Entry::Occupied(mut entry) => {
                entry.insert(todo.clone());
                if !data.dirty {
                    data.dirty = true;
                    drop(data);
                    let _ = self.commit().await;
                }
                Ok(todo)
            }
        }
    }

    async fn remove(&self, id: &str) -> Result<(), DomainError> {
        let mut data = timeout(self.lock_timeout, self.data.write())
            .await
            .map_err(|_| DomainError::Busy())?;

        match data.map.entry(id.to_string()) {
            Entry::Vacant(_) => Err(DomainError::NotFound(format!("todo not found: {:#?}", id))),
            Entry::Occupied(entry) => {
                entry.remove();
                if !data.dirty {
                    data.dirty = true;
                    drop(data);
                    let _ = self.commit().await;
                }
                Ok(())
            }
        }
    }

    async fn list(&self, owner: &str) -> Result<Vec<TodoModel>, DomainError> {
        let data = timeout(self.lock_timeout, self.data.read())
            .await
            .map_err(|_| DomainError::Busy())?;

        let mut todos: Vec<TodoModel> = data
            .map
            .values()
            .filter(|todo| todo.owner == owner)
            .cloned()
            .collect();

        todos.sort_by_key(|a| std::cmp::Reverse(a.due_date));

        Ok(todos)
    }

    async fn commit(&self) -> Result<(), DomainError> {
        let data = timeout(self.lock_timeout, self.data.read())
            .await
            .map_err(|_| DomainError::Busy())?;

        if !data.dirty {
            return Ok(());
        }
        drop(data);

        let data = Arc::clone(&self.data);

        match self.tx.try_send(Command::CommitTodo(data)) {
            Ok(_) => {}
            Err(mpsc::error::TrySendError::Full(_)) => {
                tracing::warn!("queue full; todo commit deferred");
            }
            Err(mpsc::error::TrySendError::Closed(_)) => {
                tracing::warn!("todo worker channel closed");
            }
        }
        Ok(())
    }
}

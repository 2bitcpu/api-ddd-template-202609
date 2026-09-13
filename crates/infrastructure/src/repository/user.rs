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
use domain::{DomainError, UserRepository, model::UserModel};

use crate::model::user::UserData;
use crate::repository::{constant::USER_FILE_NAME, worker::Command};

pub struct UserRepositoryImpl {
    data: Arc<RwLock<UserData>>,
    tx: mpsc::Sender<Command>,
    lock_timeout: Duration,
}

fn load() -> UserData {
    let path = config().storage.data_dir.join(USER_FILE_NAME);

    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(_) => return UserData::new(),
    };

    match serde_json::from_str::<HashMap<String, UserModel>>(&text) {
        Ok(map) => UserData { map, dirty: false },
        Err(_) => UserData::new(),
    }
}

impl UserRepositoryImpl {
    pub fn new(tx: mpsc::Sender<Command>) -> Self {
        Self {
            data: Arc::new(RwLock::new(load())),
            tx,
            lock_timeout: Duration::from_millis(config().server.lock_timeout_millis),
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn save(&self, user: UserModel) -> Result<UserModel, DomainError> {
        let mut data = timeout(self.lock_timeout, self.data.write())
            .await
            .map_err(|_| DomainError::Busy())?;

        match data.map.entry(user.account.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(user.clone());
                if !data.dirty {
                    data.dirty = true;
                    drop(data);
                    let _ = self.commit().await;
                }
                Ok(user)
            }
            Entry::Occupied(_) => Err(DomainError::Conflict(format!(
                "user already exists: {:#?}",
                user.account
            ))),
        }
    }

    async fn find(&self, account: &str) -> Result<Option<UserModel>, DomainError> {
        let data = timeout(self.lock_timeout, self.data.read())
            .await
            .map_err(|_| DomainError::Busy())?;

        Ok(data.map.get(account).cloned())
    }

    async fn replace(&self, user: UserModel) -> Result<UserModel, DomainError> {
        let mut data = timeout(self.lock_timeout, self.data.write())
            .await
            .map_err(|_| DomainError::Busy())?;

        match data.map.entry(user.account.clone()) {
            Entry::Vacant(_) => Err(DomainError::NotFound(format!(
                "user not found: {:#?}",
                user.account
            ))),
            Entry::Occupied(mut entry) => {
                entry.insert(user.clone());
                if !data.dirty {
                    data.dirty = true;
                    drop(data);
                    let _ = self.commit().await;
                }
                Ok(user)
            }
        }
    }

    async fn remove(&self, account: &str) -> Result<(), DomainError> {
        let mut data = timeout(self.lock_timeout, self.data.write())
            .await
            .map_err(|_| DomainError::Busy())?;

        match data.map.entry(account.to_string()) {
            Entry::Vacant(_) => Err(DomainError::NotFound(format!(
                "user not found: {:#?}",
                account
            ))),
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

    async fn commit(&self) -> Result<(), DomainError> {
        let data = timeout(self.lock_timeout, self.data.read())
            .await
            .map_err(|_| DomainError::Busy())?;
        if !data.dirty {
            return Ok(());
        }
        drop(data);

        let data = Arc::clone(&self.data);

        match self.tx.try_send(Command::CommitUser(data)) {
            Ok(_) => {}
            Err(mpsc::error::TrySendError::Full(_)) => {
                tracing::warn!("queue full; user commit deferred");
            }
            Err(mpsc::error::TrySendError::Closed(_)) => {
                tracing::warn!("user worker channel closed");
            }
        }
        Ok(())
    }
}

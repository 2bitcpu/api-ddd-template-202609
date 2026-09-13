use std::sync::Arc;
use tokio::{
    sync::{RwLock, mpsc, oneshot},
    time,
};

use common::config;

use crate::model::{todo::TodoData, user::UserData};
use crate::repository::constant::{TODO_FILE_NAME, USER_FILE_NAME};
pub enum Command {
    CommitUser(Arc<RwLock<UserData>>),
    CommitTodo(Arc<RwLock<TodoData>>),
    Flush(oneshot::Sender<()>),
}

pub async fn run_worker(mut rx: mpsc::Receiver<Command>) {
    while let Some(cmd) = rx.recv().await {
        match cmd {
            Command::CommitUser(data) => {
                time::sleep(time::Duration::from_secs(
                    config().storage.wait_interval_seconds,
                ))
                .await;

                let mut data = data.write().await;
                if !data.dirty {
                    continue;
                }

                let path = config().storage.data_dir.join(USER_FILE_NAME);
                let text = match serde_json::to_string(&data.map) {
                    Ok(text) => text,
                    Err(err) => {
                        tracing::error!("serialize user data failed: {err}");
                        continue;
                    }
                };
                if let Err(err) = tokio::fs::write(path, text).await {
                    tracing::error!("write user data failed: {err}");
                    continue;
                }
                tracing::debug!("user data committed");
                data.dirty = false;
            }
            Command::CommitTodo(data) => {
                time::sleep(time::Duration::from_secs(
                    config().storage.wait_interval_seconds,
                ))
                .await;

                let mut data = data.write().await;
                if !data.dirty {
                    continue;
                }

                let path = config().storage.data_dir.join(TODO_FILE_NAME);
                let text = match serde_json::to_string(&data.map) {
                    Ok(text) => text,
                    Err(err) => {
                        tracing::error!("serialize todo data failed: {err}");
                        continue;
                    }
                };
                if let Err(err) = tokio::fs::write(path, text).await {
                    tracing::error!("write todo data failed: {err}");
                    continue;
                }
                tracing::debug!("todo data committed");
                data.dirty = false;
            }
            Command::Flush(tx) => {
                let _ = tx.send(());
            }
        }
    }
}

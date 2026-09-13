use tokio::{
    sync::{mpsc, oneshot},
    time::{Duration, timeout},
};

use common::config;
use domain::{
    Repositories, {TodoRepository, UserRepository},
};

use crate::repository::{Command, TodoRepositoryImpl, UserRepositoryImpl, run_worker};

pub struct RepositoriesImpl {
    todo: TodoRepositoryImpl,
    user: UserRepositoryImpl,
    tx: mpsc::Sender<Command>,
}

impl RepositoriesImpl {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(config().storage.queue_size);
        let _worker = tokio::spawn(run_worker(rx));

        Self {
            todo: TodoRepositoryImpl::new(tx.clone()),
            user: UserRepositoryImpl::new(tx.clone()),
            tx,
        }
    }

    pub async fn flush(&self) {
        let (tx_done, rx_done) = oneshot::channel();
        if let Err(e) = self.tx.send(Command::Flush(tx_done)).await {
            tracing::error!("flush command send failed: {e}");
            return;
        }
        match timeout(Duration::from_secs(25), rx_done).await {
            Ok(Ok(())) => {}
            Ok(Err(_)) => {
                tracing::error!("flush receiver closed unexpectedly");
            }
            Err(_) => {
                tracing::error!("flush timed out after 25s");
            }
        }
    }
}

impl Repositories for RepositoriesImpl {
    fn todo(&self) -> &dyn TodoRepository {
        &self.todo
    }

    fn user(&self) -> &dyn UserRepository {
        &self.user
    }
}

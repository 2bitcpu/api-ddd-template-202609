use common::{BoxError, config};
use domain::{
    Repositories, {TodoRepository, UserRepository},
};
use fjall::{Database, KeyspaceCreateOptions, PersistMode};

use crate::repository::{TodoRepositoryImpl, UserRepositoryImpl, run_blocking};

pub struct RepositoriesImpl {
    todo: TodoRepositoryImpl,
    user: UserRepositoryImpl,
    db: Database,
}

impl RepositoriesImpl {
    pub fn new() -> Result<Self, BoxError> {
        let db = Database::builder(&config().storage.data_dir).open()?;
        let user_ks = db.keyspace("user", KeyspaceCreateOptions::default)?;
        let todo_ks = db.keyspace("todo", KeyspaceCreateOptions::default)?;

        Ok(Self {
            todo: TodoRepositoryImpl::new(todo_ks),
            user: UserRepositoryImpl::new(user_ks),
            db,
        })
    }

    pub async fn flush(&self) -> Result<(), BoxError> {
        let db = self.db.clone();
        run_blocking(move || {
            db.persist(PersistMode::SyncAll)?;
            Ok(())
        })
        .await
        .map_err(|error| Box::new(error) as BoxError)
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

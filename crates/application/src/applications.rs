use async_trait::async_trait;
use std::sync::Arc;

use crate::use_case::auth::AuthUseCase;
use crate::use_case::todo::TodoUseCase;
use domain::Repositories;

pub trait Applications: Send + Sync {
    fn auth(&self) -> &AuthUseCase;
    fn todo(&self) -> &TodoUseCase;
}

pub struct ApplicationsImpl {
    auth: AuthUseCase,
    todo: TodoUseCase,
}

impl ApplicationsImpl {
    pub fn new(repositories: Arc<dyn Repositories>) -> Self {
        Self {
            auth: AuthUseCase::new(repositories.clone()),
            todo: TodoUseCase::new(repositories),
        }
    }
}

#[async_trait]
impl Applications for ApplicationsImpl {
    fn auth(&self) -> &AuthUseCase {
        &self.auth
    }

    fn todo(&self) -> &TodoUseCase {
        &self.todo
    }
}

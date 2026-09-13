use crate::repository::{TodoRepository, UserRepository};

pub trait Repositories: Send + Sync {
    fn todo(&self) -> &dyn TodoRepository;
    fn user(&self) -> &dyn UserRepository;
}

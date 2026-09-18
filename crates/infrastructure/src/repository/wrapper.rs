use domain::DomainError;

use super::RepositoryError;

pub async fn run_blocking<T, F>(operation: F) -> Result<T, DomainError>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, RepositoryError> + Send + 'static,
{
    tokio::task::spawn_blocking(operation)
        .await
        .map_err(|error| DomainError::Unexpected(Box::new(error)))?
        .map_err(RepositoryError::into_domain)
}

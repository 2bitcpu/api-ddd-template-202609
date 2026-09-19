use domain::DomainError;

pub fn unexpected<E>(e: E) -> DomainError
where
    E: std::error::Error + Send + Sync + 'static,
{
    DomainError::Unexpected(Box::new(e))
}

pub async fn run_blocking<T, F>(operation: F) -> Result<T, DomainError>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, DomainError> + Send + 'static,
{
    tokio::task::spawn_blocking(operation)
        .await
        .map_err(unexpected)?
        .map_err(unexpected)
}

use common::BoxError;
use domain::DomainError;

pub enum RepositoryError {
    Domain(DomainError),
    External(BoxError),
}

impl From<DomainError> for RepositoryError {
    fn from(error: DomainError) -> Self {
        Self::Domain(error)
    }
}

impl From<fjall::Error> for RepositoryError {
    fn from(error: fjall::Error) -> Self {
        Self::External(Box::new(error))
    }
}

impl From<serde_json::Error> for RepositoryError {
    fn from(error: serde_json::Error) -> Self {
        Self::External(Box::new(error))
    }
}

impl RepositoryError {
    pub fn into_domain(self) -> DomainError {
        match self {
            Self::Domain(error) => error,
            Self::External(error) => DomainError::from(error),
        }
    }
}

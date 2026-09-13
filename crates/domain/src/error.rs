use common::BoxError;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum DomainError {
    NotFound(String),
    Conflict(String),
    Busy(),
    Unexpected(BoxError),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::NotFound(reason) => write!(f, "not found: {}", reason),
            DomainError::Conflict(reason) => write!(f, "conflict: {}", reason),
            DomainError::Busy() => write!(f, "Busy"),
            DomainError::Unexpected(e) => {
                write!(f, "unexpected error occurred: {}", e)
            }
        }
    }
}

impl Error for DomainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DomainError::Unexpected(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

impl From<BoxError> for DomainError {
    fn from(e: BoxError) -> Self {
        DomainError::Unexpected(e)
    }
}

impl From<serde_json::Error> for DomainError {
    fn from(e: serde_json::Error) -> Self {
        DomainError::Unexpected(Box::new(e))
    }
}

impl From<std::io::Error> for DomainError {
    fn from(e: std::io::Error) -> Self {
        DomainError::Unexpected(Box::new(e))
    }
}

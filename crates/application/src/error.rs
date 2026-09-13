use std::{error::Error, fmt};

use common::BoxError;
use domain::DomainError;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
    Unauthorized(String),
    Forbidden(String),
    ServerBusy(),
    BadRequest(String),
    DataNotFound(String),
    DataConflict(String),
    Unexpected(BoxError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Unauthorized(reason) => write!(f, "Unauthorized: {}", reason),
            AppError::Forbidden(reason) => write!(f, "Forbidden: {}", reason),
            AppError::ServerBusy() => write!(f, "Server busy"),
            AppError::BadRequest(reason) => write!(f, "Bad request: {}", reason),
            AppError::DataNotFound(reason) => write!(f, "Data not found: {}", reason),
            AppError::DataConflict(reason) => write!(f, "Data conflict: {}", reason),
            AppError::Unexpected(e) => {
                write!(f, "unexpected error occurred: {}", e)
            }
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Unexpected(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

impl From<BoxError> for AppError {
    fn from(e: BoxError) -> Self {
        AppError::Unexpected(e)
    }
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        Self::BadRequest(errors.to_string())
    }
}

impl From<DomainError> for AppError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::NotFound(reason) => Self::DataNotFound(reason),
            DomainError::Conflict(reason) => Self::DataConflict(reason),
            DomainError::Busy() => Self::ServerBusy(),
            DomainError::Unexpected(ex) => Self::Unexpected(ex),
        }
    }
}

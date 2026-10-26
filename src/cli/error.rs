
use thiserror::Error;
use crate::service::ServiceError;
use crate::repository::RepoError;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Ошибка сервиса: {0}")]
    ServiceError(ServiceError),
}

impl From<ServiceError> for CliError {
    fn from(value: ServiceError) -> Self {
        Self::ServiceError(value)
    }
}

impl From<RepoError> for CliError {
    fn from(value: RepoError) -> Self {
        Self::ServiceError(ServiceError::StorageError(value))
    }
}
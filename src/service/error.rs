use thiserror::Error;
use crate::repository::RepoError;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Некорректная дата")]
    IncorrectDate,
    #[error("Ошибка БД: {0}")]
    StorageError(RepoError),
}

impl From<RepoError> for ServiceError {
    fn from(value: RepoError) -> Self {
        Self::StorageError(value)
    }
}
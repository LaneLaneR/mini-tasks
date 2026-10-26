use rusqlite::Error;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum RepoError {
    #[error("Не удалось подключится к БД")]
    ConnectionFailed,
    #[error("Не удалось открыть или создать директорию для БД")]
    DirError,
    #[error("Ошибка при взаимодействии с БД: {0}")]
    ReqError(#[source] rusqlite::Error),
    #[error("Не найдено в БД")]
    NotFound
}

impl From<rusqlite::Error> for RepoError {
    fn from(value: Error) -> Self {
        Self::ReqError(value)
    }
}
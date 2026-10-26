use crate::repository::{RepoError, Repository};
use crate::tasks::{NewTask, Task};
use thiserror::Error;
use chrono::{prelude, Local, NaiveDate, NaiveDateTime};
use anyhow::{Context, Result};

mod error;

pub use self::error::ServiceError;
// Реализуй типаж ошибок ServiceError и трейт From для преобразования между сервисом и репозиторием

pub struct Service {
    repository: Repository,
}

impl Service {
    pub fn new(repository: Repository) -> Self {
        Self {repository}
    }

    pub fn create(&self, task: &NewTask) -> Result<(), ServiceError> {
        let task_date = task.day();
        let local_date = Local::now().date_naive();

        if *task_date < local_date {
            return Err(ServiceError::IncorrectDate);
        }

        // Знак вопроса автоматом дергает From
        self.repository.create(task)?;

        Ok(())
    }

    pub fn get_by_date(&self, date: NaiveDate) -> Result<Vec<Task>, ServiceError> {
        Ok(self.repository.get_by_date(date)?)
    }

    pub fn delete(&self, id: u32) -> Result<(), ServiceError> {
        Ok(self.repository.delete(id)?)
    }

    pub fn execute(&self, id: u32, date: NaiveDateTime) -> Result<(), ServiceError> {
        Ok(self.repository.execute(id, date)?)
    }
}
// Приложение должно:
// Принимать

use directories::ProjectDirs;
use rusqlite::{params, Connection, Error};
use std::fs;
use chrono::{NaiveDate, NaiveDateTime};
use crate::tasks::{Task, NewTask};
use thiserror::Error;

mod error;

pub use error::RepoError;

pub struct Repository {
    connection: Connection,
}

impl Repository {
    pub fn new() -> Result<Self, RepoError> {
        let prod_dirs = match ProjectDirs::from("org", "LaneLaneR", "mini-tasks") {
            Some(p) => p,
            None => return Err(RepoError::DirError)
        };

        let data_dir = prod_dirs.data_dir();

        if let Err(_) = fs::create_dir_all(data_dir) {
            return Err(RepoError::DirError);
        }

        let db_path = data_dir.join("tasks.sqlite");

        let conn = Connection::open(&db_path)?;

        let output = Self { connection: conn };

        output.init()?;

        Ok(output)
    }

    fn init(&self) -> Result<(), Error> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            date            TEXT NOT NULL,
            title           TEXT NOT NULL,
            description     TEXT NOT NULL,
            completed       INTEGER NOT NULL DEFAULT 0,
            completed_at    TEXT
        );",
            [],
        )?;

        Ok(())
    }

    pub fn create(&self, task: &NewTask) -> Result<(), RepoError> {
        let created_at = task.day().to_string();
        let title = task.title();
        let description = task.description();

         self.connection.execute(
            "INSERT INTO tasks (date, title, description)
                VALUES (
                    ?,
                    ?,
                    ?
                );",
            params![created_at, title.to_string(), description.to_string()]
        )?;

        Ok(())
    }

    pub fn get_by_date(&self, date: chrono::NaiveDate) -> Result<Vec<Task>, RepoError> {
        let date = date.to_string();

        let mut stmt = self.connection.prepare(
            "SELECT id, date, title, description, completed, completed_at FROM tasks
            WHERE date = ?;"
        )?;

        let user_iter = stmt.query_map(params![date], |row| {
            Ok(Task {
                id: row.get(0)?,
                date: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                completed: row.get(4)?,
                completed_at: row.get(5)?,
            })
        })?;

        let mut result = vec![];

        for i in user_iter {
            // map_err() преобразует одну ошибку в другую, не затрагивая успешный результат
            // До map_err Ok(task) Err(err) после Ok(task) Err(RepoError::ReqError)
            result.push(i.map_err(RepoError::ReqError)?);
        }

        Ok(result)
    }

    pub fn delete(&self, id: u32) -> Result<(), RepoError> {
        self.connection.execute(
            "DELETE FROM tasks WHERE id = ?;",[id]
        )?;
        Ok(())
    }

    pub fn execute(&self, id: u32, date: NaiveDateTime) -> Result<(), RepoError> {
        let rows = self.connection.execute(
            "UPDATE tasks SET completed = 1, completed_at = ? WHERE id = ?",
            params![date.to_string(), id],
        )?;

        if rows == 0 {
            Err(RepoError::NotFound)
        } else {
            Ok(())
        }
    }
}

// date: YYYY-MM-DD
// table
// CREATE TABLE IF NOT EXISTS tasks (
// id INTEGER PRIM)

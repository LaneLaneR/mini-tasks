pub mod models;

use chrono::prelude::*;


pub struct Task {
    pub id: u32,
    pub date: String,
    pub title: String,
    pub description: String,
    pub completed: u32,
    pub completed_at: Option<String>,
}

#[derive(Debug)]
pub struct NewTask {
    title: String,
    description: String,
    day: NaiveDate,
}

impl NewTask {
    pub fn new(title: String, description: String, day: NaiveDate) -> Self {
        Self {
            title,
            description,
            day,
        }
    }
}

// Getters
impl NewTask {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn day(&self) -> &NaiveDate {
        &self.day
    }
}

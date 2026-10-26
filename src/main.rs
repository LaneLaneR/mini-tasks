use chrono::Local;
use clap::Parser;

use tasks::Task;

use crate::{cli::{Cli, Commands}, repository::Repository, service::Service, tasks::NewTask};

mod repository;
mod tasks;
mod service;
mod cli;

fn print_tasks(tasks: &[Task], is_long: bool) {
    if tasks.is_empty() {
        println!("На выбранную дату задач нет.");
        return;
    }

    for task in tasks {
        let status = if task.completed != 0 { "☑" } else { "□" };

        if is_long {
            println!("#{}: {} {} - {}\n\t{}", task.id, status, task.title, task.date, task.description);

        } else {
            println!("{} {} - {}", status, task.title, task.date);
        }
    }
}

fn main() -> Result<(), cli::CliError> {
    let cli = Cli::parse();
    let repository = Repository::new()?;
    let service = Service::new(repository);

    match cli.command {
        Commands::Add { title, description, day } => {
            let new_task = NewTask::new(title, description, day);
            service.create(&new_task)?;
            println!("Задача успешно добавлена: \"{}\" на {}", new_task.title(), new_task.day());
        }

        Commands::Show(args) => {
            let date = args.date.unwrap_or_else(|| Local::now().date_naive());
            let tasks = service.get_by_date(date)?;

            print_tasks(&tasks, args.long);
        }

        Commands::Delete { id } => {
            service.delete(id)?;
            println!("Задача #{} удалена.", id);
        }

        Commands::Execute { id } => {
            let now = Local::now().naive_local();
            service.execute(id, now)?;
            println!("Задача #{} отмечена как выполненная.", id);
        }
    }


    Ok(())
}

// database: ~/local/share/mini-tasks/tasks.db

// Приложение должно:
// Принимать аргументы командной строки и возвращать определенные данные за день из базы данных
// Что мы храним в БД:
// Столбцы: id, title, description, day, completed
// Формат day: YYYY-MM-DD

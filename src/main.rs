#![allow(dead_code)]
mod cli;
mod todo;
use clap::Parser;

use chrono::{DateTime, NaiveDateTime, Utc};
use cli::Args;
use todo::{CreateTodo, Priority, Status, TodoList, UpdateTodo};

fn parse_priority(priority: &str) -> Result<Priority, String> {
    match priority.to_lowercase().as_str() {
        "low" => Ok(Priority::Low),
        "medium" => Ok(Priority::Medium),
        "high" => Ok(Priority::High),
        _ => Err(format!("Invalid priority: {}", priority)),
    }
}

fn parse_status(status: &str) -> Result<Status, String> {
    match status.to_lowercase().as_str() {
        "pending" => Ok(Status::Pending),
        "inprogress" => Ok(Status::InProgress),
        "completed" => Ok(Status::Completed),
        _ => Err(format!("Invalid status: {}", status)),
    }
}

fn parse_date(date_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    NaiveDateTime::parse_from_str(date_str, "%Y/%m/%d %H:%M:%S")
        .map(|dt| dt.and_local_timezone(Utc).unwrap())
}

fn main() {
    let args = Args::parse();

    let mut todo_list = TodoList::new();

    if let Some(title) = args.add {
        let priority = args
            .priority
            .as_deref()
            .map(parse_priority)
            .transpose()
            .unwrap_or_else(|_| Some(Priority::Medium));

        let due_date = args
            .due_date
            .as_deref()
            .map(parse_date)
            .transpose()
            .unwrap_or_else(|e| {
                eprintln!("Invalid date format: {}", e);
                None
            });

        let new_todo = CreateTodo {
            title,
            description: args.description,
            due_date,
            priority: priority.unwrap_or(Priority::Medium),
        };

        let id = todo_list.add(new_todo);
        println!("Added new TODO with ID: {}", id);
    } else if let Some(id) = args.update {
        let mut update_todo = UpdateTodo {
            title: None,
            description: args.description,
            due_date: None,
            status: None,
            priority: None,
        };

        if let Some(priority) = args.priority {
            update_todo.priority = Some(parse_priority(&priority).unwrap_or_else(|e| {
                eprintln!("{}", e);
                Priority::Medium
            }));
        }

        if let Some(status) = args.status {
            update_todo.status = Some(parse_status(&status).unwrap_or_else(|e| {
                eprintln!("{}", e);
                Status::InProgress
            }));
        }

        if let Some(date_str) = args.due_date {
            update_todo.due_date = parse_date(&date_str).ok();
        }

        match todo_list.update(id, update_todo) {
            Some(_) => println!("Updated TODO with ID: {}", id),
            None => println!("No TODO found with ID: {}", id),
        }
    } else if let Some(id) = args.delete {
        match todo_list.delete(id) {
            Some(_) => println!("Deleted TODO with ID: {}", id),
            None => println!("No TODO found with ID: {}", id),
        }
    } else if let Some(id) = args.show {
        if let Some(todo) = todo_list.get(id) {
            println!("ID: {}", todo.id);
            println!("Title: {}", todo.title);
            println!("Description: {:?}", todo.description);
            println!("Status: {:?}", todo.status);
            println!("Priority: {:?}", todo.priority);
            println!("Created at: {}", todo.created_at);
            println!("Due date: {:?}", todo.due_date);
        } else {
            println!("No TODO found with ID: {}", id);
        }
    } else if args.list || std::env::args().len() == 1 {
        let todos = todo_list.list();
        if todos.is_empty() {
            println!("TODO list is empty.");
        } else {
            for todo in todos {
                println!(
                    "ID: {}, Title: {}, Status: {:?}, Priority: {:?}",
                    todo.id, todo.title, todo.status, todo.priority
                );
            }
        }
    } else {
        println!("Usage: rustodo [OPTIONS]");
        println!("Use --help to see available options.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn scenario_test() {
        // 新しいTODOリストを作成
        let mut todo_list = TodoList::new();

        // シナリオ1: 新しいTODOを追加
        let id1 = todo_list.add(CreateTodo {
            title: "Implement TODO CLI".to_string(),
            description: Some("Create a Rust CLI for managing TODOs".to_string()),
            due_date: None,
            priority: Priority::High,
        });

        // 追加されたTODOを検証
        let todo1 = todo_list.todos.get(&id1).unwrap();
        assert_eq!(todo1.title, "Implement TODO CLI");
        assert_eq!(todo1.status, Status::InProgress);
        assert_eq!(todo1.priority, Priority::High);

        // シナリオ2: TODOを更新
        let update_result = todo_list.update(
            id1,
            UpdateTodo {
                title: None,
                description: None,
                due_date: Some(Utc::now() + Duration::days(7)),
                status: Some(Status::Completed),
                priority: None,
            },
        );
        assert!(update_result.is_some());

        // 更新されたTODOを検証
        let updated_todo1 = todo_list.todos.get(&id1).unwrap();
        assert_eq!(updated_todo1.status, Status::Completed);
        assert!(updated_todo1.due_date.is_some());

        // シナリオ3: 別のTODOを追加
        let id2 = todo_list.add(CreateTodo {
            title: "Write tests".to_string(),
            description: None,
            due_date: Some(Utc::now() + Duration::days(3)),
            priority: Priority::Medium,
        });

        // シナリオ4: すべてのTODOをリスト化
        let todos = todo_list.list();
        assert_eq!(todos.len(), 2);

        // シナリオ5: TODOを削除
        let deleted_todo = todo_list.delete(id1);
        assert!(deleted_todo.is_some());
        assert_eq!(deleted_todo.unwrap().id, id1);

        // 削除後のリストを検証
        let remaining_todos = todo_list.list();
        assert_eq!(remaining_todos.len(), 1);
        assert_eq!(remaining_todos[0].id, id2);
    }
}

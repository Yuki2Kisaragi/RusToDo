mod cli;
mod todo;
mod todo_list;

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, TimeZone};
use chrono_tz::Tz;
use clap::Parser;
use cli::Args;
use todo::{CreateTodo, Priority, Status, UpdateTodo};
use todo_list::TodoList;

fn parse_priority(priority: &str) -> Result<Priority> {
    priority
        .parse()
        .map_err(|_| anyhow!("Invalid status: {}. (Low, Medium, High)", priority))
}

fn parse_status(status: &str) -> Result<Status> {
    status.parse().map_err(|_| {
        anyhow!(
            "Invalid status: {}. (Pending, InProgress, Completed)",
            status
        )
    })
}

fn parse_date(date_str: &str, tz: &Tz) -> Result<DateTime<Tz>> {
    let naive = chrono::NaiveDateTime::parse_from_str(date_str, "%Y/%m/%d %H:%M:%S")?;
    Ok(tz.from_local_datetime(&naive).single().unwrap())
}

fn get_local_timezone() -> Tz {
    use iana_time_zone::get_timezone;
    use std::str::FromStr;

    match get_timezone() {
        Ok(tz_string) => Tz::from_str(&tz_string).unwrap_or(Tz::UTC),
        Err(_) => Tz::UTC,
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let local_tz = get_local_timezone();
    let todo_list = TodoList::new(local_tz).context("Failed to create TodoList")?;

    if let Some(title) = args.add {
        let priority = args
            .priority
            .as_deref()
            .map(parse_priority)
            .unwrap_or(Ok(Priority::Medium))?;

        let due_date = args
            .due_date
            .as_deref()
            .map(|date_str| parse_date(date_str, &local_tz))
            .transpose()?;

        let new_todo = CreateTodo {
            title,
            description: args.description,
            due_date,
            priority,
        };

        let id = todo_list.add(new_todo)?;
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
            update_todo.priority = Some(parse_priority(&priority)?);
        }

        if let Some(status) = args.status {
            update_todo.status = Some(parse_status(&status)?);
        }

        if let Some(date_str) = args.due_date {
            update_todo.due_date = Some(parse_date(&date_str, &local_tz)?);
        }

        todo_list.update(id, update_todo)?;
        println!("Updated TODO with ID: {}", id);
    } else if let Some(id) = args.delete {
        todo_list.delete(id)?;
        println!("Deleted TODO with ID: {}", id);
    } else if let Some(id) = args.show {
        let todo = todo_list.get(id)?;
        println!("ID: {}", todo.id);
        println!("Title: {}", todo.title);
        println!("Description: {:?}", todo.description);
        println!("Status: {:?}", todo.status);
        println!("Priority: {:?}", todo.priority);
        println!("Created at: {}", todo.created_at);
        println!("Due date: {:?}", todo.due_date);
    } else if args.list || std::env::args().len() == 1 {
        let todos = todo_list.list()?;
        if todos.is_empty() {
            println!("TODO list is empty.");
        } else {
            for todo in todos {
                println!(
                    "ID: {}, Title: {}, Status: {:?}, Priority: {:?}, Due: {:?}",
                    todo.id, todo.title, todo.status, todo.priority, todo.due_date
                );
            }
        }
    } else {
        println!("Usage: rustodo [OPTIONS]");
        println!("Use --help to see available options.");
    }

    Ok(())
}
#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn scenario_test() {
        // 新しいTODOリストを作成

        // シナリオ1: 新しいTODOを追加
        // 追加されたTODOを検証

        // シナリオ2: TODOを更新
        // 更新されたTODOを検証

        // シナリオ3: 別のTODOを追加

        // シナリオ4: すべてのTODOをリスト化

        // シナリオ5: TODOを削除
        // 削除後のリストを検証
    }
}

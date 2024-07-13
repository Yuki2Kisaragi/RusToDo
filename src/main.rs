#![allow(dead_code)]
mod cli;
mod todo;
use clap::Parser;

use chrono::{NaiveDateTime, Utc};
use cli::Args;
use std::collections::HashMap;
use todo::{CreateTodo, Priority, Status, Todo, UpdateTodo};

struct TodoList {
    todos: HashMap<u32, Todo>,
    next_id: u32,
}

impl TodoList {
    fn new() -> Self {
        Self {
            todos: HashMap::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, create_todo: CreateTodo) -> u32 {
        let id = self.next_id;
        let todo = Todo::new(
            id,
            create_todo.title,
            create_todo.description,
            create_todo.due_date,
            create_todo.priority,
        );
        self.todos.insert(id, todo);
        self.next_id += 1;
        id
    }

    fn update(&mut self, id: u32, update_todo: UpdateTodo) -> Option<()> {
        if let Some(todo) = self.todos.get_mut(&id) {
            if let Some(title) = update_todo.title {
                todo.title = title;
            }
            if let Some(description) = update_todo.description {
                todo.description = Some(description);
            }
            if let Some(due_date) = update_todo.due_date {
                todo.due_date = Some(due_date);
            }
            if let Some(status) = update_todo.status {
                todo.status = status;
            }
            if let Some(priority) = update_todo.priority {
                todo.priority = priority;
            }
            Some(())
        } else {
            None
        }
    }

    fn delete(&mut self, id: u32) -> Option<Todo> {
        self.todos.remove(&id)
    }

    fn list(&self) -> Vec<&Todo> {
        self.todos.values().collect()
    }
}

fn main() {
    let args = Args::parse();
    let mut todo_list = TodoList::new();

    // Add a new TODO
    todo_list.add(CreateTodo {
        title: "Implement TODO CLI".to_string(),
        description: Some("Create a Rust CLI for managing TODOs".to_string()),
        due_date: None,
        priority: Priority::High,
    });

    todo_list.add(CreateTodo {
        title: "Update TODO CLI".to_string(),
        description: Some("Update a Rust CLI for managing TODOs".to_string()),
        due_date: Some(
            NaiveDateTime::parse_from_str("2024/07/31 12:00:00", "%Y/%m/%d %H:%M:%S")
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap(),
        ),
        priority: Priority::Medium,
    });

    if args.list || std::env::args().len() == 1 {
        // TODOリストを表示
        let todos = todo_list.list();
        if todos.is_empty() {
            println!("TODOリストは空です。");
        } else {
            for todo in todos {
                println!(
                    "ID: {}, タイトル: {}, 状態: {:?}, 優先度: {:?}",
                    todo.id, todo.title, todo.status, todo.priority
                );
            }
        }
    } else {
        println!("使用方法: rustodo [OPTIONS]");
        println!("オプションを確認するには --help を使用してください。");
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

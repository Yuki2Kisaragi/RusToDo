use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: Priority,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateTodo {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: Priority,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub status: Option<Status>,
    pub priority: Option<Priority>,
}

impl Todo {
    pub fn new(
        id: u32,
        title: String,
        description: Option<String>,
        due_date: Option<DateTime<Utc>>,
        priority: Priority,
    ) -> Todo {
        Self {
            id,
            title,
            description,
            status: Status::InProgress,
            created_at: Utc::now(),
            due_date,
            priority,
        }
    }
}

pub struct TodoList {
    pub todos: HashMap<u32, Todo>,
    next_id: u32,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            todos: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, create_todo: CreateTodo) -> u32 {
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

    pub fn update(&mut self, id: u32, update_todo: UpdateTodo) -> Option<()> {
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

    pub fn delete(&mut self, id: u32) -> Option<Todo> {
        self.todos.remove(&id)
    }

    pub fn list(&self) -> Vec<&Todo> {
        self.todos.values().collect()
    }
    pub fn get(&self, id: u32) -> Option<&Todo> {
        self.todos.get(&id)
    }
}

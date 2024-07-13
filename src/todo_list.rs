use crate::todo::{CreateTodo, Todo, UpdateTodo};
use chrono_tz::Tz;
use std::collections::HashMap;

pub struct TodoList {
    todos: HashMap<u32, Todo>,
    next_id: u32,
    timezone: Tz,
}

impl TodoList {
    pub fn new(timezone: Tz) -> Self {
        Self {
            todos: HashMap::new(),
            next_id: 1,
            timezone,
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
            self.timezone,
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

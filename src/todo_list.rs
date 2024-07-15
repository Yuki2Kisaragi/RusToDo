use crate::todo::{CreateTodo, Status, Todo, UpdateTodo};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use rusqlite::{params, Connection, Result as SqliteResult};

pub struct TodoList {
    conn: Connection,
    timezone: Tz,
}

impl TodoList {
    pub fn new(timezone: Tz) -> Result<Self> {
        let conn = Connection::open("todos.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL,
                due_date TEXT,
                priority TEXT NOT NULL
            )",
            [],
        )?;
        Ok(Self { conn, timezone })
    }

    pub fn add(&self, create_todo: CreateTodo) -> Result<u32> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO todos (title, description, status, created_at, due_date, priority)
             VALUES (?, ?, ?, ?, ?, ?)",
        )?;
        stmt.execute(params![
            create_todo.title,
            create_todo.description,
            Status::InProgress.to_string(),
            Utc::now().with_timezone(&self.timezone).to_rfc3339(),
            create_todo
                .due_date
                .map(|d| d.with_timezone(&self.timezone).to_rfc3339()),
            create_todo.priority.to_string(),
        ])?;
        Ok(self.conn.last_insert_rowid() as u32)
    }

    pub fn update(&self, id: u32, update_todo: UpdateTodo) -> Result<()> {
        let todo = self.get(id)?;
        let mut stmt = self.conn.prepare(
            "UPDATE todos SET title = ?, description = ?, status = ?, due_date = ?, priority = ?
             WHERE id = ?",
        )?;
        stmt.execute(params![
            update_todo.title.unwrap_or(todo.title),
            update_todo.description.or(todo.description),
            update_todo.status.unwrap_or(todo.status).to_string(),
            update_todo
                .due_date
                .or(todo.due_date)
                .map(|d| d.with_timezone(&self.timezone).to_rfc3339()),
            update_todo.priority.unwrap_or(todo.priority).to_string(),
            id,
        ])?;
        Ok(())
    }

    pub fn delete(&self, id: u32) -> Result<()> {
        self.conn
            .execute("DELETE FROM todos WHERE id = ?", params![id])?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Todo>> {
        let mut stmt = self.conn.prepare("SELECT * FROM todos")?;
        let todos = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                status: row.get::<_, String>(3)?.parse().unwrap(),
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&self.timezone),
                due_date: row.get::<_, Option<String>>(5)?.map(|d| {
                    DateTime::parse_from_rfc3339(&d)
                        .unwrap()
                        .with_timezone(&self.timezone)
                }),
                priority: row.get::<_, String>(6)?.parse().unwrap(),
            })
        })?;
        todos
            .collect::<SqliteResult<Vec<Todo>>>()
            .context("Failed to collect todos")
    }

    pub fn get(&self, id: u32) -> Result<Todo> {
        let mut stmt = self.conn.prepare("SELECT * FROM todos WHERE id = ?")?;
        stmt.query_row(params![id], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                status: row.get::<_, String>(3)?.parse().unwrap(),
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&self.timezone),
                due_date: row.get::<_, Option<String>>(5)?.map(|d| {
                    DateTime::parse_from_rfc3339(&d)
                        .unwrap()
                        .with_timezone(&self.timezone)
                }),
                priority: row.get::<_, String>(6)?.parse().unwrap(),
            })
        })
        .context("Todo not found")
    }
}

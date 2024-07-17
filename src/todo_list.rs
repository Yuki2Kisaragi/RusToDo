use crate::todo::{CreateTodo, Status, Todo, UpdateTodo};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use rusqlite::{params, Connection, Result as SqliteResult};
use std::path::Path;

pub struct TodoList {
    conn: Connection,
    timezone: Tz,
}

impl TodoList {
    pub fn new<P: AsRef<Path>>(timezone: Tz, db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todo::Priority;
    use chrono::{TimeZone, Utc};
    use tempfile::tempdir;

    fn setup_test_db() -> (TodoList, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let tz = Tz::UTC;
        (TodoList::new(tz, db_path.to_str().unwrap()).unwrap(), dir)
    }

    #[test]
    fn test_add_todo() {
        let (todo_list, _dir) = setup_test_db();
        let new_todo = CreateTodo {
            title: "Test Todo".to_string(),
            description: Some("Test Description".to_string()),
            due_date: Some(
                Utc.with_ymd_and_hms(2023, 12, 31, 23, 59, 59)
                    .unwrap()
                    .with_timezone(&todo_list.timezone),
            ),
            priority: Priority::Medium,
        };
        let id = todo_list.add(new_todo).unwrap();
        assert_eq!(id, 1);
    }

    #[test]
    fn test_get_todo() {
        let (todo_list, _dir) = setup_test_db();
        let new_todo = CreateTodo {
            title: "Test Todo".to_string(),
            description: Some("Test Description".to_string()),
            due_date: Some(
                Utc.with_ymd_and_hms(2023, 12, 31, 23, 59, 59)
                    .unwrap()
                    .with_timezone(&todo_list.timezone),
            ),
            priority: Priority::Medium,
        };
        let id = todo_list.add(new_todo).unwrap();

        let todo = todo_list.get(id).unwrap();
        assert_eq!(todo.title, "Test Todo");
        assert_eq!(todo.description, Some("Test Description".to_string()));
        assert_eq!(todo.priority, Priority::Medium);
    }

    #[test]
    fn test_update_todo() {
        let (todo_list, _dir) = setup_test_db();
        let new_todo = CreateTodo {
            title: "Test Todo".to_string(),
            description: None,
            due_date: None,
            priority: Priority::Low,
        };
        let id = todo_list.add(new_todo).unwrap();

        let update_todo = UpdateTodo {
            title: Some("Updated Todo".to_string()),
            description: Some("Updated Description".to_string()),
            due_date: Some(
                Utc.with_ymd_and_hms(2023, 12, 31, 23, 59, 59)
                    .unwrap()
                    .with_timezone(&todo_list.timezone),
            ),
            status: Some(Status::Completed),
            priority: Some(Priority::High),
        };
        todo_list.update(id, update_todo).unwrap();

        let updated_todo = todo_list.get(id).unwrap();
        assert_eq!(updated_todo.title, "Updated Todo");
        assert_eq!(
            updated_todo.description,
            Some("Updated Description".to_string())
        );
        assert_eq!(updated_todo.status, Status::Completed);
        assert_eq!(updated_todo.priority, Priority::High);
    }

    #[test]
    fn test_delete_todo() {
        let (todo_list, _dir) = setup_test_db();
        let new_todo = CreateTodo {
            title: "Test Todo".to_string(),
            description: None,
            due_date: None,
            priority: Priority::Low,
        };
        let id = todo_list.add(new_todo).unwrap();

        todo_list.delete(id).unwrap();

        assert!(todo_list.get(id).is_err());
    }

    #[test]
    fn test_list_todos() {
        let (todo_list, _dir) = setup_test_db();
        let todo1 = CreateTodo {
            title: "Todo 1".to_string(),
            description: None,
            due_date: None,
            priority: Priority::Low,
        };
        let todo2 = CreateTodo {
            title: "Todo 2".to_string(),
            description: None,
            due_date: None,
            priority: Priority::Medium,
        };
        todo_list.add(todo1).unwrap();
        todo_list.add(todo2).unwrap();

        let todos = todo_list.list().unwrap();
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].title, "Todo 1");
        assert_eq!(todos[1].title, "Todo 2");
    }
}

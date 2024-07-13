use chrono::{DateTime, Utc};

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

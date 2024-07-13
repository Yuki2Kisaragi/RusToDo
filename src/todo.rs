use chrono::DateTime;
use chrono_tz::Tz;
use std::str::FromStr;

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

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Tz>,
    pub due_date: Option<DateTime<Tz>>,
    pub priority: Priority,
}

#[derive(Debug, Clone)]
pub struct CreateTodo {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Tz>>,
    pub priority: Priority,
}

#[derive(Debug, Clone)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Tz>>,
    pub status: Option<Status>,
    pub priority: Option<Priority>,
}

impl Todo {
    pub fn new(
        id: u32,
        title: String,
        description: Option<String>,
        due_date: Option<DateTime<Tz>>,
        priority: Priority,
        tz: Tz,
    ) -> Todo {
        Self {
            id,
            title,
            description,
            status: Status::InProgress,
            created_at: chrono::Local::now().with_timezone(&tz),
            due_date,
            priority,
        }
    }
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Status::Pending),
            "inprogress" => Ok(Status::InProgress),
            "completed" => Ok(Status::Completed),
            _ => Err(()),
        }
    }
}

impl FromStr for Priority {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            _ => Err(()),
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Pending => "Pending",
            Status::InProgress => "InProgress",
            Status::Completed => "Completed",
        }
        .to_string()
    }
}

impl ToString for Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        }
        .to_string()
    }
}

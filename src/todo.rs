use chrono::DateTime;
use chrono_tz::Tz;
use core::fmt;
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
    pub name: String,
    pub text: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Tz>,
    pub due_date: Option<DateTime<Tz>>,
    pub priority: Priority,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "text: {:?}", self.text)?;
        writeln!(f, "Status: {:?}", self.status)?;
        writeln!(f, "Priority: {:?}", self.priority)?;
        writeln!(f, "Created at: {}", self.created_at)?;
        writeln!(f, "Due date: {:?}", self.due_date)
    }
}

#[derive(Debug, Clone)]
pub struct CreateTodo {
    pub name: String,
    pub text: Option<String>,
    pub due_date: Option<DateTime<Tz>>,
    pub priority: Priority,
}

#[derive(Debug, Clone)]
pub struct UpdateTodo {
    pub name: Option<String>,
    pub text: Option<String>,
    pub due_date: Option<DateTime<Tz>>,
    pub status: Option<Status>,
    pub priority: Option<Priority>,
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(Status::Pending),
            "inprogress" | "in progress" => Ok(Status::InProgress),
            "completed" => Ok(Status::Completed),
            _ => Err(format!("Invalid status: {}", s)),
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

impl FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            _ => Err(format!("Invalid priority: {}", s)),
        }
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

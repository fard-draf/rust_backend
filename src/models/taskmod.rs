use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(id: u32, title: String, status: TaskStatus) -> Self {
        Self { id, title, status }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskStatus {
    InProgress,
    Done,
    Paused,
}

impl TaskStatus {
    pub fn to_str(&self) -> &str {
        match self {
            TaskStatus::InProgress => "task in progress",
            TaskStatus::Done => "task completed",
            TaskStatus::Paused => "task paused",
        }
    }
}

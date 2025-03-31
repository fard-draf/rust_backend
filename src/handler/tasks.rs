use crate::models::taskmod::{Task, TaskStatus};

use axum::{Extension, Json};
use sqlx::PgPool;

pub async fn tasks_handler() -> Json<Vec<Task>> {
    let tasks = vec![
        Task::new(1, "Learning Rust and Axum".into(), TaskStatus::InProgress),
        Task::new(
            2,
            "Build backend with Rust and Axum".into(),
            TaskStatus::Done,
        ),
    ];
    Json(tasks)
}

// pub async fn create_task(
//     Extension: Extension<PgPool>,

// );
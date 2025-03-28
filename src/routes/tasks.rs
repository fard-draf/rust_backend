use crate::models::task::Task;
use axum::Json;

pub async fn tasks_handler() -> Json<Vec<Task>> {
    let tasks = vec![
        Task::new(1, "Learning Rust and Axum".into(), false),
        Task::new(2, "Build backend with Rust and Axum".into(), true),
    ];
    Json(tasks)
}

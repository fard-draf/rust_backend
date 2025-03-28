pub mod health;
mod tasks;

use super::routes::{health::healthcheck_handler, tasks::tasks_handler};
use axum::{Router, routing::get};

pub fn create_router() -> Router {
    Router::new()
        .route("/healthcheck", get(healthcheck_handler))
        .route("/tasks", get(tasks_handler))
}

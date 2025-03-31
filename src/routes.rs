use std::sync::Arc;

use axum::{Extension, Router, routing::get };
use tower_http::trace::TraceLayer;


use crate::handler::{tasks::tasks_handler, health::healthcheck_handler};
use crate::AppState;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_route = Router::new()     
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state));

    Router::new()
        .route("/healthcheck", get(healthcheck_handler))
        .route("/tasks", get(tasks_handler))
        .nest("/api", api_route)
}
mod models;
mod routes;

use axum::{Router, routing::get};
use routes::{health::healthcheck_handler, tasks::tasks_handler};
use tokio::net::TcpListener;

// use crate::routes::create_router;

#[tokio::main]
async fn main() {
    let app = Router::new()
                        .route("/healthcheck", get(healthcheck_handler))
                        .route("/tasks", get(tasks_handler));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on 127.0.0.1:3000.");

    axum::serve(listener, app).await.unwrap();
}

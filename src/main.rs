mod models;
mod routes;

use axum::Router;
use tokio::net::TcpListener;

use crate::routes::create_router;

#[tokio::main]
async fn main() {
    let app = Router::new();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on 127.0.0.1:3000.");

    axum::serve(listener, app).await.unwrap();
}

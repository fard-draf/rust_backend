mod config;
mod db;
mod models;
mod handler;
mod routes;

use std::sync::Arc;

use config::Config;
use db::DBClient;
use hyper::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, Method};
use routes::create_router;
use sqlx::{pool, postgres::PgPoolOptions};
use axum::{http::HeaderValue, routing::get, Router};
use dotenv::dotenv;
use handler::{health::healthcheck_handler, tasks::tasks_handler};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database successful.");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database..{:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT]);

    let db_client = DBClient::new(pool); 
    let app_state = AppState {
        env: config.clone(),
        db_client,
    };

    let app = create_router(Arc::new(app_state.clone())).layer(cors.clone());

    println!(
        "{}",
        format!("Server is running on http://localhost:{}", config.port)
    );

    let listener = TcpListener::bind(format!("0.0.0.0:{}", &config.port))
    .await
    .unwrap();
    

    axum::serve(listener, app).await.unwrap();
}

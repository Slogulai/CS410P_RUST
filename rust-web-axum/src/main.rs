//https://medium.com/@raditzlawliet/build-crud-rest-api-with-rust-and-mysql-using-axum-sqlx-d7e50b3cd130

use std::sync::Arc;

use dotenv::dotenv;
use sqlx::mysql::{MySqlPool,MySqlPoolOptions};

use axum::{response::IntoResponse, routing::get, Json, Router};
use tokio::net::TcpListener;

pub struct AppState {
    db: MySqlPool,
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Restful question API!");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to the database!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };


    let app = Router::new()
        .route("/healthcheck", get(health_check))
        .with_state(Arc::new(AppState { db: pool.clone() }));

    println!("Server running at 0.0.0.0:8000");

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

pub async fn health_check() -> impl IntoResponse { 
    const MESSAGE: &str = "I'm alive!";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE,
    });

    Json(json_response)
}
mod handler;
mod response;
mod question;
mod route;

#[allow(unused)]
use handler::*;
#[allow(unused)]
use response::*;
#[allow(unused)]
use question::*;
#[allow(unused)]
use route::*;


#[allow(unused)]
use axum::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
    error_handling::HandleError,
    // Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post, put, Router},
};
#[allow(unused)]
use std::net::SocketAddr;
use ::serde::{Deserialize, Serialize};
#[allow(unused)]
use chrono::prelude::*;

use std::fmt;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;


use route::create_router;
use tower_http::cors::Corslayer;

//~~~~~ASYNC STUFF~~~~~~

//https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/
async fn health_check() -> impl IntoResponse {
    const MESSAGE: &str = "I'm alive!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE,

    });

    Json(json_response)
}
#[allow(unused)]
#[tokio::main]
async fn main() {
    //let app = Router::new().route("/", get(health_check));
    let cors = Corslayer::new()
        .allow_any_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router().layer(cors);

    println!("Starting server on port 3000...");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

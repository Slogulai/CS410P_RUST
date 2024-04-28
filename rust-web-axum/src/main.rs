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

extern crate headers;

#[allow(unused)]
use axum::{
    http::{HeaderValue, Method},
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
#[allow(unused)]
use tower::{ServiceBuilder, ServiceExt, Service};
#[allow(unused)]
use tower_http::cors::{Any, CorsLayer};
use headers::*;

//~~~~~ASYNC STUFF~~~~~~

//https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/
#[allow(unused)]
#[tokio::main]
async fn main() {
    //let app = Router::new().route("/", get(health_check));
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([HeaderName::from_lowercase(b"content-type").unwrap()]);

    let app = create_router().layer(cors);

    println!("Starting server on port 3000...");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

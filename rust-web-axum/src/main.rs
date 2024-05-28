mod handler;
mod response;
mod question;
mod route;
mod questionbase;

//#[allow(unused)]
//use handler::{create_question_handler, get_question_handler, health_check, question_list_handler};
//#[allow(unused)]
//use response::{GenericRepsonse, QuestionData, SingleQuestionResponse, QuestionListResponse};
//#[allow(unused)]
use question::*;
use questionbase::*;

extern crate headers;

use axum::http::{HeaderValue, Method, /*Extension, StatusCode */};
use axum::Extension; // Replace axum::http::Extension with axum::Extension
use std::fmt;
use std::collections::HashMap;
use route::create_router;
use tower_http::cors::CorsLayer;
use ::serde::{Deserialize, Serialize};
use headers::*;
use std::env;
use sqlx::{PgPool, Row, FromRow};
use sqlx::postgres::PgRow;

//Unused gang
#[allow(unused)]
use std::sync::Arc;
#[allow(unused)]
use tokio::sync::Mutex;
#[allow(unused)]
use std::fs::File;

//https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/
#[tokio::main]
async fn main() {
    let pool = question_db().await.expect("Failed to create pool");
    set_database(&pool).await.expect("Failed to set database");
   
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([HeaderName::from_lowercase(b"content-type").unwrap()]);

    let router = create_router().await.expect("Failed to create router");
    let app = router.layer(cors);

    println!("Starting server on 127.0.0.1:3000...");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

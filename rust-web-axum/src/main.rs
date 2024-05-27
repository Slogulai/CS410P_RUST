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

extern crate headers;

//#[allow(unused)]
use axum::{
    http::{HeaderValue, Method/*, StatusCode */},
    //response::{IntoResponse, Json},
    //routing::Rejection,
    //error_handling::HandleError,
    // Json, Router,
    //extract::{Path, State},
    //routing::{delete, get, post, put, Router},
};
use std::fmt;
//#[allow(unused)]
//use std::convert::Infallible;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use route::create_router;
use tower_http::cors::CorsLayer;
use ::serde::{Deserialize, Serialize};
use headers::*;
use std::fs::File;
use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use std::env;


const DB_URL: &str = env!("DATABASE_URL");
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

//~~~~~~Thingies to Remember~~~~~~
//Persistant store
//random questions
//adding questions
//updating questions
//database integration
//get docker desktop



//https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/
#[tokio::main]
async fn main() {
    let file = File::open("questions.json").unwrap_or_else(|_| File::create("questions.json").unwrap());
    let reader = BufReader::new(file);
    let initial_state: HashMap<String, Question> = serde_json::from_reader(reader).unwrap_or_default();

    // Use the existing questions as the initial state
    let db = Arc::new(Mutex::new(initial_state));
   
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([HeaderName::from_lowercase(b"content-type").unwrap()]);

    let app = create_router().layer(cors);

    println!("Starting server on 127.0.0.1:3000...");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

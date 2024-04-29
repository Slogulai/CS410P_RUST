mod handler;
mod response;
mod question;
mod route;
mod questionbase;

#[allow(unused)]
use handler::*;
#[allow(unused)]
use response::*;
#[allow(unused)]
use question::*;
#[allow(unused)]
use route::*;
#[allow(unused)]
use questionbase::*;

extern crate headers;

#[allow(unused)]
use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Json},
    //routing::Rejection,
    //error_handling::HandleError,
    // Json, Router,
    //extract::{Path, State},
    //routing::{delete, get, post, put, Router},
};
use std::fmt;
#[allow(unused)]
use std::convert::Infallible;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use route::create_router;
use tower_http::cors::CorsLayer;
use ::serde::{Deserialize, Serialize};
use headers::*;

#[allow(unused)]
use std::io::{Error, ErrorKind};
#[allow(unused)]
use std::fs::File;

//use std::str::FromStr;
//use tower::{ServiceBuilder, ServiceExt, Service};
//use std::net::SocketAddr;

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
    let store = Store::new();
    let _db = Arc::new(Mutex::new(store.question_map.clone()));

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

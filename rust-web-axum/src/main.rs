#[allow(unused)]
use std::net::SocketAddr;
#[allow(unused)]
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post, put, Router},
    error_handling::HandleError,
   // Json, Router,
};

#[allow(unused)]
use chrono::prelude::*;
use::serde::{Serialize, Deserialize};
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::fmt;
#[allow(unused)]
use tokio::sync::Mutex;
#[allow(unused)]
use std::sync::Arc;



//~~~~~~QUESTIONS STUFF~~~~~~~~
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    id : QuestionId,
    title : String,
    content : String,
    tags : Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct QuestionId(String);
impl FromStr for QuestionId {
    type Err = Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(
                Error::new(ErrorKind::InvalidInput, "No ID provided!")
        ),
        }
    }
}
#[allow(unused)]
impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Self {
            id,
            title,
            content,
            tags,
        }
    }
    /*
    //Rust isnt liking this function
    fn update_title(&self, new_title: String) -> Self {
        Question::new(self.id, new_title, self.content, self.tags)
    }
    */
}
impl std::fmt::Debug for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}\n{}\n{:?}", self.id.0, self.title, self.content, self.tags)
    }
}
impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

//#[allow(unused)]
pub type DB = Arc<Mutex<Vec<Question>>>;
pub fn question_db() -> DB {
    Arc::new(Mutex::new(Vec::new()))
}
#[derive(Debug, Deserialize, Serialize)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateQuestionSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
}
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
    let app = Router::new().route("/", get(health_check));

    println!("Starting server on port 3000...");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

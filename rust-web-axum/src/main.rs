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
use::serde::Serialize;
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
struct Question {
    id : QuestionId,
    title : String,
    content : String,
    tags : Option<Vec<String>>,
}
struct QuestionId(String);
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
impl std::fmt::Debug for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

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

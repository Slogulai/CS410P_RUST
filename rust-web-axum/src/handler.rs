//https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/


#[allow(unused)]
use std::error;

#[allow(unused)]
use crate::*;

#[allow(unused)]
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
#[allow(unused)]
use serde_json::from_str;
#[allow(unused)]
use uuid::Uuid;

#[allow(unused)]
use crate::{
    question::{QueryOptions, Question, UpdateQuestionSchema, DB},
    response::{QuestionListResponse, QuestionData, SingleQuestionResponse},
};

pub async fn health_check() -> impl IntoResponse {
    const MESSAGE: &str = "I'm alive!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE,

    });

    Json(json_response)
}
#[allow(unused)]
pub async fn question_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<DB>,
) -> impl IntoResponse {
    let questions = db.lock().await;

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let questions: Vec<Question> = questions
        .clone()
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();
    let json_response = QuestionListResponse {
        status: "success".to_string(),
        results: questions.len(),
        questions,
    };

    Json(json_response)
}

pub async fn create_question_handler ( 
    State(db): State<DB>,
    Json(body): Json<Question>,  
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>  {
    let mut vec = db.lock().await;

    if let Some(question) = vec.iter().find(|question| question.title == body.title) {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Question with ID {} already exists", question.title),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let id = body.id.clone();
    let title = body.title.clone();
    let content = body.content.clone();
    let tags = body.tags.clone();

    vec.push(body);

    let question = Question::new(id, title, content, tags);

    let json_response = SingleQuestionResponse {
        status: "success".to_string(),
        data: question,
    };


    Ok((StatusCode::CREATED, Json(json_response)))
     
}

pub async fn get_question_handler(
    Path(id): Path<String>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let vec = db.lock().await;

    if let Some(question) = vec.iter().find(|question| question.id == id) {
        let json_response = SingleQuestionResponse {
            status: "success".to_string(),
            data: question.clone(),
        };
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "error",
        "message": format!("Question with ID {} not found", id),
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

pub async fn edit_question_handler(
    Path(id): Path<String>,
    State(db): State<DB>,
    Json(body): Json<UpdateQuestionSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(question) = vec.iter_mut().find(|question| Some(question.id.clone()) == Some(id.clone())) {
        question.title = body.title.clone().unwrap();
        question.content = body.content.clone().unwrap();
        question.tags = body.tags.clone();

        let json_response = SingleQuestionResponse {
            status: "success".to_string(),
            data: question.clone(),
        };

        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "error",
        "message": format!("Question with ID {} not found", id),
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

pub async fn delete_question_handler(
    Path(id): Path<String>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(pos) = vec.iter().position(|question| Some(question.id.clone()) == Some(id.clone())) {
        vec.remove(pos);
        return Ok((StatusCode::NO_CONTENT, Json(serde_json::json!({}))));
    }

    let error_response = serde_json::json!({
        "status": "error",
        "message": format!("Question with ID {} not found", id),
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}


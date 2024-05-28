//https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/

//use std::error;
//use serde_json::from_str;
//use uuid::Uuid;

use crate::*;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    //routing::get,
    response::IntoResponse,
    Json,
};
//use anyhow::Error as BoxError;
use std::fs::File;
use std::io::Read;

use crate::{
    question::{QueryOptions, Question,/* UpdateQuestionSchema, */ DB},
    response::{QuestionListResponse,/* QuestionData, */SingleQuestionResponse},
};

pub async fn health_check() -> impl IntoResponse {
    const MESSAGE: &str = "I'm alive!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE,

    });

    Json(json_response)
}

//#[allow(unused)]
/*
pub async fn get_questions(store: Store) -> Result<Vec<Question>, BoxError> {
    let res: Vec<Question> = store.question_map.values().cloned().collect();
    Ok(res)
}
*/
pub async fn get_question_handler(
    Path(id): Path<i32>,
    State(db): State<MyDatabase>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match db.get_question(id).await {
        Ok(question) => {
            let json_response = SingleQuestionResponse {
                status: "success".to_string(),
                data: question,
            };
            Ok((StatusCode::OK, Json(json_response)))
        }
        Err(err) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Failed to get question: {}", err),
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}


pub async fn question_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<MyDatabase>,
) -> impl IntoResponse {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match db.get_questions(limit, offset).await {
        Ok(questions) => {
            let json_response = QuestionListResponse {
                status: "success".to_string(),
                results: questions.len(),
                questions,
            };

            Json(json_response)
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "Failed to fetch questions",
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

pub async fn create_question_handler(
    Json(body): Json<Question>,
    State(db): State<MyDatabase>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match db.create_question(&body.title, &body.content, &body.tags).await {
        Ok(question) => {
            let json_response = SingleQuestionResponse {
                status: "success".to_string(),
                data: question,
            };

            Ok((StatusCode::CREATED, Json(json_response)))
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Question with title {} already exists", body.title),
            });
            Err((StatusCode::CONFLICT, Json(error_response)))
        }
    }
}


/*
pub async fn edit_question_handler(
    Path(id): Path<String>,
    State(db): State<DB>,
    Json(body): Json<UpdateQuestionSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>{
    let id = id.to_string();
    let mut hash_map = db.lock().await;

    if let Some(question) = hash_map.iter_mut().find(|question| Some(question.1.id.clone()) == Some(id.clone())) {
        question.1.title = body.title.clone().unwrap();
        question.1.content = body.content.clone().unwrap();
        question.1.tags = body.tags.clone();

        let json_response = SingleQuestionResponse {
            status: "success".to_string(),
            data: question.1.clone(),
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
    let mut hash_map = db.lock().await;

    if hash_map.iter().any(|question| Some(question.1.id.clone()) == Some(id.clone())) {
        hash_map.remove_entry(&id);
        return Ok((StatusCode::NO_CONTENT, Json(serde_json::json!({}))));
    }

    let error_response = serde_json::json!({
        "status": "error",
        "message": format!("Question with ID {} not found", id),
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

*/
//https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/


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
    question::{QueryOptions, Question, UpdateQuestionSchema, DB, QuestionId},
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

) {

}

pub async fn edit_question_handler(

) {

}

pub async fn delete_question_handler(

) {

}


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
use uuid::Uuid;

#[allow(unused)]
use crate::{
    question::{QueryOptions, Question, UpdateQuestionSchema, DB},
    response::{QuestionListResponse, QuestionData, SingleQuestionResponse},
};

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
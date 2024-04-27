use crate::mode::Todo;
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericRepsonse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct QuestionResponse {
    pub question: Question,
}

#[derive(Serialize, Debug)]
pub struct SingleQuestionResponse {
    pub status: String,
    pub data: QuestionData,
}

#[derive(Serialize, Debug)]
pub struct QuestionListResponse {
    pub status: String,
    pub results: usize,
    pub questions: Vec<Question>,
}
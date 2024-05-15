//https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/
use crate::*;
//use crate::QuestionData;
//use serde::Serialize;

#[derive (Deserialize)]
pub struct IdParam {
    pub id: String,
}
#[derive(Serialize)]
pub struct GenericRepsonse {
    pub status: String,
    pub message: String,
}
#[derive(Serialize, Debug)]
pub struct QuestionData {
    pub question: Question,
}
#[derive(Serialize, Debug)]
pub struct SingleQuestionResponse {
    pub status: String,
    pub data: Question,
}
#[derive(Serialize, Debug)]
pub struct QuestionListResponse {
    pub status: String,
    pub results: usize,
    pub questions: HashMap<String, Question>,
}
#[derive(Deserialize)]
pub struct UpdateQuestionForm {
    pub title: String,
    pub content: String,
    pub tags: String,
}
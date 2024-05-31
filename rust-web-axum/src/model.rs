//https://medium.com/@raditzlawliet/build-crud-rest-api-with-rust-and-mysql-using-axum-sqlx-d7e50b3cd130
use serde::{Deserialize, Serialize};

//Sqlx Struct
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct QuestionModel {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub tags: String,
}

//Json Struct
#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionModelResponse {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub tags: String,
}
//https://medium.com/@raditzlawliet/build-crud-rest-api-with-rust-and-mysql-using-axum-sqlx-d7e50b3cd130
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateQuestionSchema {
    pub question: String,
    pub answer: String,
    pub tags: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateQuestionSchema {
    pub question: Option<String>,
    pub answer: Option<String>,
    pub tags: Option<String>,
}
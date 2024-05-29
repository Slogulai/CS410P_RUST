//https://medium.com/@raditzlawliet/build-crud-rest-api-with-rust-and-mysql-using-axum-sqlx-d7e50b3cd130
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Question {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub tags: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewQuestion {
    pub question: String
}

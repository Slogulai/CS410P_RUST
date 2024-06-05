//https://medium.com/@raditzlawliet/build-crud-rest-api-with-rust-and-mysql-using-axum-sqlx-d7e50b3cd130

use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use tera::{Context, Tera};
use serde_json::json;

use crate::{
    model::{QuestionModel, QuestionModelResponse},
    schema::{CreateQuestionSchema, FilterOptions, UpdateQuestionSchema},
    AppState,
};

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}

fn to_question_response(question: &QuestionModel) -> QuestionModelResponse {
    QuestionModelResponse {
        id: question.id.to_owned(),
        question: question.question.to_owned(),
        answer: question.answer.to_owned(),
        tags: question.tags.to_owned(),
    }
}

pub async fn question_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(1000);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let questions = sqlx::query_as!(
        QuestionModel,
        r#"SELECT id, question, answer, tags FROM questions ORDER by id LIMIT ? OFFSET ?"#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let question_responeses = questions.iter()
    .map(|question| to_question_response(&question))
    .collect::<Vec<QuestionModelResponse>>();

    // Create a Tera instance and add your templates directory to it
    let tera = match Tera::new("assets/*") {
        Ok(t) => t,
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "status": "error",
                "message": format!("Template error: {}", e),
            }))))
        }
    };

    // Create a context and add your data to it
    let mut context = Context::new();
    context.insert("questions", &question_responeses);

    // Render your template with the context
    let rendered = match tera.render("all_questions.html", &context) {
        Ok(r) => r,
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "status": "error",
                "message": format!("Rendering error: {}", e),
            }))))
        }
    };

    Ok(axum::response::Html(rendered))
}

pub async fn create_question_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateQuestionSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = uuid::Uuid::new_v4().to_string();
    let query_result = sqlx::query(
        r#"INSERT INTO questions (id, question, answer, tags) VALUES (?, ?, ?, ?)"#,
    ).bind(id.clone())
    .bind(body.question.to_string())
    .bind(body.answer.to_string())
    .execute(&data.db)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status": "error",
                "message": "Question already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }

    let question = sqlx::query_as!(QuestionModel, r#"SELECT id, question, answer, tags FROM questions WHERE id = ?"#, id)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}", e)}))
            )
        })?;

    let question_response = serde_json::json!({
        "status": "ok",
        "data": serde_json::json!({
            "quesion": to_question_response(&question)
        }),
    });

    Ok(Json(question_response))
}

pub async fn get_question_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        QuestionModel,
        r#"SELECT id, question, answer, tags FROM questions WHERE id = ?"#,
        id.to_string()
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(question) => {
            let question_response = serde_json::json!({
                "status": "ok",
                "data": serde_json::json!({
                    "question": to_question_response(&question)
                }),
            });

            return Ok(Json(question_response));
        }
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Question with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}", e)})),
            ));
        }
    };
}

pub async fn edit_question_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateQuestionSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        QuestionModel,
        r#"SELECT id, question, answer, tags FROM questions WHERE id = ?"#,
        id.to_string(),
    )
    .fetch_one(&data.db)
    .await;

    let question = match query_result {
        Ok(question) => question,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Question with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })),
            ));
        }
    };

    let update_result = 
        sqlx::query(r#"UPDATE question SET question = ?, answer = ?, tags = ? WHERE id = ?"#)
            .bind(body.question.to_owned().unwrap_or_else(|| question.question.clone()))
            .bind(body.answer.to_owned().unwrap_or_else(|| question.answer.clone()))
            .bind(body.tags.to_owned().unwrap_or_else(|| question.tags.clone()))
            .bind(id.to_string())
            .execute(&data.db)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": format!("{:?}", e)
                    })),
                )
            })?;

    if update_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Failed to update question with ID: {}", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let updated_quesiton = sqlx::query_as!(
        QuestionModel,
        r#"SELECT * FROM questions WHERE id = ?"#,
        id.to_string()
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": format!("{:?}", e)
            })),
        )
    })?;

    let question_response = serde_json::json!({
        "status": "ok",
        "data": serde_json::json!({
            "question": to_question_response(&updated_quesiton)
        }),
    });

    Ok(Json(question_response))
}

pub async fn delete_question_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query!(r#"DELETE FROM questions WHERE id = ?"#,id.to_string())
        .execute(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })),
            )
        })?;

    if query_result.rows_affected() == 0 {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Failed to delete question with ID: {}", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}
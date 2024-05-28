use crate::*;

use crate::{
    question::{QueryOptions, Question,},
    response::{QuestionListResponse, SingleQuestionResponse},
    questionbase::MyDatabase,
};

pub async fn health_check() -> impl IntoResponse {
    const MESSAGE: &str = "I'm alive!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE,

    });

    Json(json_response)
}

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


pub async fn get_random_question_handler(
    State(db): State<MyDatabase>,
) -> impl IntoResponse {
    let questions: Vec<Question> = sqlx::query_as("SELECT * FROM questions")
        .fetch_all(&*db)
        .await
        .map_err(|err| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Failed to fetch questions: {}", err),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    if questions.is_empty() {
        let error_response = serde_json::json!({
            "status": "error",
            "message": "No questions found",
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }
    let mut rng = rand::thread_rng();
    let random_question = questions.choose(&mut rng).unwrap();

    let question_template = fs::read_to_string("assets/question_template.html").await.expect("Unable to read file");
    let html_response = question_template
        .replace("{id}", &random_question.id.to_string())
        .replace("{title}", &random_question.title)
        .replace("{content}", &random_question.content)
        .replace("{tags}", &random_question.tags.as_ref().unwrap_or(&Vec::new()).join(", "));

    Ok(axum::response::Html(html_response))
}





pub async fn get_all_questions_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<MyDatabase>,
) -> Result<axum::response::Html<String>, axum::http::Response<axum::Json<serde_json::Value>>> {
    let questions: Vec<Question> = sqlx::query_as("SELECT * FROM questions")
        .fetch_all(&*db)
        .await
        .map_err(|err| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Failed to fetch questions: {}", err),
            });
            axum::http::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Json(error_response))
                .unwrap()
        })?;

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(1000);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let questions: Vec<Question> = questions.into_iter().skip(offset).take(limit).collect();

    let questions_template = fs::read_to_string("assets/question_template.html").await.expect("Unable to read file");
    let mut questions_html = String::new();

    for question in &questions {
        let question_html = questions_template
            .replace("{id}", &question.id.to_string())
            .replace("{title}", &question.title)
            .replace("{content}", &question.content)
            .replace("{tags}", &question.tags.as_ref().unwrap_or(&Vec::new()).join(", "));
        questions_html.push_str(&question_html);
    }

    let questions_template = fs::read_to_string("assets/questions_template.html").await.expect("Unable to read file");
    let html_response = questions_template.replace("{questions}", &questions_html);

    Ok(axum::response::Html(html_response))
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
    State(db): State<MyDatabase>,
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
    State(db): State<MyDatabase>,
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
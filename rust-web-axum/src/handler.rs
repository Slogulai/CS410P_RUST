//https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/
use crate::*;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
    //routing::get,
};
use std::io::Read;
use std::fs;
use axum::response::Html;

use crate::{
    question::{QueryOptions, Question,/* UpdateQuestionSchema, */ DB},
    //response::{/*QuestionListResponse, QuestionData, */SingleQuestionResponse},
};

pub async fn welcome_handler() -> Result<Html<String>, StatusCode> {
    match fs::read_to_string("assets/welcome.html") {
        Ok(contents) => Ok(Html(contents)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_question_handler(
    Path(id): Path<String>,
    State(_db): State<DB>,
) -> impl IntoResponse {
    let mut file = File::open("questions.json").map_err(|err| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Failed to read file: {}", err),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Failed to read file: {}", err),
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }
    let questions: HashMap<String, Question> = serde_json::from_str(&contents).unwrap();

    if let Some(question) = questions.get(&id) {
        let question_template = fs::read_to_string("assets/question_template.html").expect("Unable to read file");
        let html_response = question_template
            .replace("{id}", &id)
            .replace("{title}", &question.title)
            .replace("{content}", &question.content)
            .replace("{tags}", &question.tags.as_ref().unwrap_or(&Vec::new()).join(", "));

        return Ok(axum::response::Html(html_response));
    }

    let error_response = serde_json::json!({
        "status": "error",
        "message": format!("Question with ID {} not found", id),
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}


pub async fn get_random_question_handler(
    State(_db): State<DB>,
) -> impl IntoResponse {
    let mut file = File::open("questions.json").map_err(|err| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Failed to read file: {}", err),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Failed to read file: {}", err),
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }
    let questions: HashMap<String, Question> = serde_json::from_str(&contents).unwrap();

    let ids: Vec<String> = questions.keys().cloned().collect();
    let mut rng = rand::thread_rng();
    let random_id = ids[rng.gen_range(0..ids.len())].clone();

    if let Some(question) = questions.get(&random_id) {
        let question_template = fs::read_to_string("assets/question_template.html").expect("Unable to read file");
        let html_response = question_template
            .replace("{id}", &random_id)
            .replace("{title}", &question.title)
            .replace("{content}", &question.content)
            .replace("{tags}", &question.tags.as_ref().unwrap_or(&Vec::new()).join(", "));

        return Ok(axum::response::Html(html_response));
    }

    let error_response = serde_json::json!({
        "status": "error",
        "message": format!("Question with ID {} not found", random_id),
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

pub async fn get_all_questions_handler(
    opts: Option<Query<QueryOptions>>,
    State(_db): State<DB>,
) -> impl IntoResponse {
    let file = File::open("questions.json").expect("Unable to open file");
    let reader = BufReader::new(file);
    let questions: HashMap<String, Question> = serde_json::from_reader(reader).expect("Unable to parse JSON");

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let questions: HashMap<String, Question> = questions
        .into_iter()
        .skip(offset)
        .take(limit)
        .map(|(key, value)| (key.to_string(), value))
        .collect();

    let questions_template = fs::read_to_string("assets/question_template.html").expect("Unable to read file");
    let mut questions_html = String::new();
    let _html_response = questions_template.replace("{questions}", &questions_html);

    for (id, question) in &questions {
        let question_html = questions_template
            .replace("{id}", id)
            .replace("{title}", &question.title)
            .replace("{content}", &question.content)
            .replace("{tags}", &question.tags.as_ref().unwrap_or(&Vec::new()).join(", "));
        questions_html.push_str(&question_html);
    }

    let questions_template = fs::read_to_string("assets/questions_template.html").expect("Unable to read file");
    let html_response = questions_template.replace("{questions}", &questions_html);

    axum::response::Html(html_response)
}



pub async fn add_question_form_handler() -> Result<Html<String>, StatusCode> {
    match fs::read_to_string("assets/tell.html") {
        Ok(contents) => Ok(Html(contents)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}




pub async fn create_question_handler(
    State(db): State<DB>,
    Json(body): Json<Question>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut questions = db.lock().await;

    if questions.iter().any(|question| question.1.id == body.id) {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Question with ID {} already exists", body.id),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    questions.insert(body.id.clone(), body.clone());

    let file = File::create("questions.json").map_err(|err| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Failed to write to file: {}", err),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;
    serde_json::to_writer_pretty(file, &*questions).map_err(|err| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Failed to serialize questions: {}", err),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let json_response = serde_json::json!({
        "status": "success",
        "message": format!("Successfully created question with ID {}", body.id),
    });
    Ok(Json(json_response))
}


/*

//These functions all compile and run below. I may use them later
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
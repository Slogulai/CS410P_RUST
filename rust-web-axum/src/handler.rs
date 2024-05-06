//https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/


//~~~~May use these later~~~~
//use std::error;
//use serde_json::from_str;
//use uuid::Uuid;
//use anyhow::Error as BoxError;
/*
pub async fn get_questions(store: Store) -> Result<Vec<Question>, BoxError> {
    let res: Vec<Question> = store.question_map.values().cloned().collect();
    Ok(res)
}
*/

use crate::*;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
    //routing::get,
};
use std::fs::File;
use std::io::Read;

use crate::{
    question::{QueryOptions, Question,/* UpdateQuestionSchema, */ DB},
    response::{/*QuestionListResponse, QuestionData, */SingleQuestionResponse},
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
        let mut html_response = String::from("<h1>Question:</h1>");
        html_response.push_str(&format!("<h2>Question {}:</h2>", id));
        html_response.push_str(&format!("<p>{}</p>", question.title));
        html_response.push_str(&format!("<p>{}</p>", question.content));
        html_response.push_str("<p>Tags: ");
        if let Some(tags) = &question.tags {
            html_response.push_str(&tags.join(", "));
        }
        html_response.push_str("</p>");

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
        let mut html_response = String::from("<h1>Question:</h1>");
        html_response.push_str(&format!("<h2>Question {}:</h2>", random_id));
        html_response.push_str(&format!("<p>{}</p>", question.title));
        html_response.push_str(&format!("<p>{}</p>", question.content));
        html_response.push_str("<p>Tags: ");
        if let Some(tags) = &question.tags {
            html_response.push_str(&tags.join(", "));
        }
        html_response.push_str("</p>");

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

    let mut html_response = String::from("<h1>All Questions:</h1>");

    for (id, question) in &questions {
        html_response.push_str(&format!("<h2>Question {}:</h2>", id));
        html_response.push_str(&format!("<p>{}</p>", question.title));
        html_response.push_str(&format!("<p>{}</p>", question.content));
        html_response.push_str("<p>Tags: ");
        if let Some(tags) = &question.tags {
            html_response.push_str(&tags.join(", "));
        }
        html_response.push_str("</p>");
    }

    axum::response::Html(html_response)
}

pub async fn create_question_handler ( 
    State(db): State<DB>,
    Json(body): Json<Question>,  
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)>  {
    let mut question = db.lock().await;

    if let Some(question) = question
        .iter()
        .find(|question| question.1.title == body.title) {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Question with ID {} already exists", question.1.title),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let id = body.id.clone();
    let title = body.title.clone();
    let content = body.content.clone();
    let tags = body.tags.clone();

    question.insert(body.id.to_string(), body);

    let question = Question::new(id, title, content, tags);

    let json_response = SingleQuestionResponse {
        status: "success".to_string(),
        data: question,
    };


    Ok((StatusCode::CREATED, Json(json_response)))
     
}

//These functions all compile and run below. I may use them later
/*
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
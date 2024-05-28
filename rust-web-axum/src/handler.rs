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



pub async fn get_edit_question_handler(
    Query(IdParam { id, .. }): Query<IdParam>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let file = File::open("questions.json").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut questions_vec: Vec<Question> = serde_json::from_reader(reader).expect("Unable to parse JSON");
    questions_vec.sort_by(|a, b| a.id.cmp(&b.id));
    let mut hash_map = db.lock().await;
    *hash_map = questions_vec
        .into_iter()
        .map(|question| (question.id.clone(), question))
        .collect();

    if let Some(question) = hash_map.iter().find(|question| Some(question.1.id.clone()) == Some(id.clone())) {
        let edit_question_template = fs::read_to_string("assets/edit_question.html").expect("Unable to read file");
        let html_response = edit_question_template
            .replace("{id}", &id)
            .replace("{title}", &question.1.title)
            .replace("{content}", &question.1.content)
            .replace("{tags}", &question.1.tags.as_ref().unwrap_or(&Vec::new()).join(", "))
            .replace("{id}", &id);

        return Ok(axum::response::Html(html_response));
    }

    let error_response = serde_json::json!({
        "status": "error",
        "message": format!("Question with ID {} not found", id),
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

/*
pub async fn edit_question_handler(
    Query(IdParam { id, .. }): Query<IdParam>,
    State(db): State<DB>,
    Form(body): Form<UpdateQuestionForm>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut hash_map = db.lock().await;

    if let Some(question) = hash_map.get(&id) {
        let mut question_clone = question.clone();
        question_clone.title = body.title.clone();
        question_clone.content = body.content.clone();
        question_clone.tags = Some(vec![body.tags.clone()]);

        hash_map.insert(id.clone(), question_clone);

        let file = File::create("questions.json").map_err(|err| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Failed to write to file: {}", err),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

        let values: Vec<_> = hash_map.values().collect();
        serde_json::to_writer_pretty(&file, &values).map_err(|err| {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Failed to serialize questions: {}", err),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

        let json_response = SingleQuestionResponse {
            status: "success".to_string(),
            data: hash_map.get(&id).unwrap().clone(),
        };

        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "error",
        "message": format!("Question with ID {} not found", id),
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}


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
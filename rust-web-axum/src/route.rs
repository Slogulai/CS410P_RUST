use axum::{
    routing::{get, post},
    Router,
};
use crate::{
    handler::{
        create_question_handler, 
       // delete_question_handler, 
       // edit_question_handler,
        get_question_handler,
        health_check,
        question_list_handler,
        //get_questions,
    },
    questionbase::question_db,
};

pub async fn create_router() -> Result<Router, sqlx::Error> {
    let db = question_db().await?;
    Ok(
        Router::new()
            .route("/", get(welcome_handler))
            .route("/question", get(get_random_question_handler))
            .route("/questions", get(get_all_questions_handler))
            .route("/question/:id", get(get_question_handler))
            .route("/add_question", post(create_question_handler).get(add_question_form_handler))
            .route("/edit_question", get(get_edit_question_handler))
            .route("/edit_question", post(edit_question_handler))
            //.route("/delete_question", post(delete_question_handler))
            .with_state(db)
    )
}/*
pub fn create_router(db: Arc<Mutex<HashMap<String, Question>>>) -> Router {
    Router::new()
        .route("/", get(welcome_handler))
        .route("/question", get(get_random_question_handler))
        .route("/questions", get(get_all_questions_handler))
        .route("/question/:id", get(get_question_handler))
        .route("/add_question", post(create_question_handler).get(add_question_form_handler))
        .route("/edit_question", get(get_edit_question_handler))
        .route("/edit_question", post(edit_question_handler))
        //.route("/delete_question", post(delete_question_handler))
        .with_state(db)
}
*/
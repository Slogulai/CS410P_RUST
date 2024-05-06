use axum::{
    routing::{get, post},
    Router,
};
use crate::{
    handler::{
       // delete_question_handler, 
       // edit_question_handler,
        health_check,
        get_question_handler,
        get_all_questions_handler,
        get_random_question_handler,
        add_question_form_handler,
        create_question_handler, 
        //get_questions,
    },
    question,
};
pub fn create_router() -> Router {
    let db = question::question_db();
    Router::new()
        .route("/healthchecker", get(health_check))
        .route("/questions", get(get_all_questions_handler))
        .route("/question/:id", get(get_question_handler))
        .route("/random_question", get(get_random_question_handler))
        .route("/add_question", post(create_question_handler).get(add_question_form_handler))
        .with_state(db)
}
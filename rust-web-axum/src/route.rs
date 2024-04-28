use axum::{
    routing::{get, post},
    Router,
};
#[allow(unused)]
use crate::{
    handler::{
        create_question_handler, 
        delete_question_handler, 
        edit_question_handler,
        get_question_handler,
        health_check,
        question_list_handler,
    },
    question,
    *,
};
pub fn create_router() -> Router {
    let db = question::question_db();
    Router::new()
        .route("/api/healthchecker", get(health_check))
        .route("/api/questions", 
            post(create_question_handler).get(question_list_handler),
        )
        .route("/api/questions/:id", 
            get(get_question_handler)
            .patch(edit_question_handler)
            .delete(delete_question_handler),
        )
        .with_state(db)
}
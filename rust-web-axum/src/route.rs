use axum::{
    routing::{get, post},
    Router,
};
use crate::{
    handler::{
       // delete_question_handler, 
        edit_question_handler,
        welcome_handler,
        get_question_handler,
        get_all_questions_handler,
        get_random_question_handler,
        add_question_form_handler,
        create_question_handler, 
        get_edit_question_handler,
        //get_questions,
    },
    *,
};
pub fn create_router(db: Arc<Mutex<HashMap<String, Question>>>) -> Router {
    Router::new()
        .route("/", get(welcome_handler))
        .route("/question", get(get_random_question_handler))
        .route("/questions", get(get_all_questions_handler))
        .route("/question/:id", get(get_question_handler))
        .route("/add_question", post(create_question_handler).get(add_question_form_handler))
        .route("/edit_question", get(get_edit_question_handler))
        .route("/edit_question", post(edit_question_handler))
        .with_state(db)
}
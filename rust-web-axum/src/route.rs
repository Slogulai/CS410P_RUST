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
    question,
    //handler, // Import the handler module
};
pub fn create_router() -> Router {
    let db = question::question_db();
    Router::new()
        .route("/healthchecker", get(health_check))
        .route("/questions", 
            post(create_question_handler).get(question_list_handler),
        )
        .route("/questions/:id", 
            get(get_question_handler)
          //  .patch(edit_question_handler)
           // .delete(delete_question_handler),
        )
        .route("/questions_data", get(get_question_handler))
        //.route("/questions_data", get(handler::get(get_questions))) // Fix: Wrap get_questions in handler::get
        .with_state(db)
}
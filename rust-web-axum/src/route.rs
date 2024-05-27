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
            .route("/healthchecker", get(health_check))
            .route("/questions", 
                post(create_question_handler).get(question_list_handler),
            )
            .route("/questions/:id", 
                get(get_question_handler)
            )
            .route("/questions_data", get(get_question_handler))
            .with_state(db)
    )
}
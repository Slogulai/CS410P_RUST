//https://medium.com/@raditzlawliet/build-crud-rest-api-with-rust-and-mysql-using-axum-sqlx-d7e50b3cd130
use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_question_handler,
        delete_question_handler,
        health_check_handler,
        question_list_handler,
        get_question_handler,
        edit_question_handler,
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthcheck", get(health_check_handler))
        .route("/questions", post(create_question_handler))
        .route("/questions", get(question_list_handler))
        .route("/question/:id", get(get_question_handler))
        .route(
            "/questions/:id",
            get(get_question_handler)
                .patch(edit_question_handler)
                .delete(delete_question_handler),
        )
        .with_state(app_state)
}
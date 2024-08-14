use axum::{routing::post, Router};

use crate::handlers::todo_handler;

pub fn create_app() -> Router {
    Router::new().route("/todos", post(todo_handler::create))
}

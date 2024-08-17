use axum::{routing::post, Router};
use sqlx::PgPool;

use crate::handlers::todo_handler;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub fn create_app(pool: PgPool) -> Router {
    let state = AppState { pool };
    Router::new()
        .route("/todos", post(todo_handler::create))
        .with_state(state)
}

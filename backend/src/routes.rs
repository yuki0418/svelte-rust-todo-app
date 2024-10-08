use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::handlers::todo_handler;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub fn create_app(pool: PgPool) -> Router {
    let state = AppState { pool };
    Router::new()
        .route(
            "/todos",
            get(todo_handler::get_list).post(todo_handler::create),
        )
        .route(
            "/todos/:id",
            get(todo_handler::find_by_id)
                .put(todo_handler::complete)
                .delete(todo_handler::archive),
        )
        .with_state(state)
}

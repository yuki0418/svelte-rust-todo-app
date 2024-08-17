use axum::http::StatusCode;

use crate::{
    routes::AppState,
    services::todo_service::{self, NewTodo},
};

#[derive(serde::Deserialize)]
pub struct CreateRequest {
    pub title: String,
}

impl From<CreateRequest> for NewTodo {
    fn from(val: CreateRequest) -> Self {
        NewTodo { title: val.title }
    }
}

pub async fn create(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Json(request): axum::extract::Json<CreateRequest>,
) -> StatusCode {
    let _ = todo_service::create(state, request.into());

    StatusCode::CREATED
}

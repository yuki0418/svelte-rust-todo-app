use axum::{http::StatusCode, Json};

use crate::{
    routes::AppState,
    services::todo_service::{self, NewTodo, Todo},
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
    let res = todo_service::create(state, request.into()).await;

    match res {
        Ok(_) => StatusCode::CREATED,
        Err(err) => {
            println!("Failed to create todo: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn get_list(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> (StatusCode, Json<Vec<Todo>>) {
    let res = todo_service::get_list(state).await;

    match res {
        Ok(todo_list) => (StatusCode::OK, Json(todo_list)),
        Err(err) => {
            println!("Failed to get todo list: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

pub async fn complete(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> StatusCode {
    let res = todo_service::complete(state, id).await;

    match res {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            println!("Failed to complete todo: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

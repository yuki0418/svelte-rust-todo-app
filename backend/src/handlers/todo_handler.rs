use axum::http::StatusCode;

#[derive(serde::Deserialize)]
pub struct CreateRequest {
    pub title: String,
}

pub async fn create(
    axum::extract::Json(request): axum::extract::Json<CreateRequest>,
) -> StatusCode {
    println!("creating todo with title: {}", request.title);
    StatusCode::CREATED
}

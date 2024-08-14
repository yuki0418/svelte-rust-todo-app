use axum::http::StatusCode;

pub async fn create(// axum::extract::Json(request): axum::extract::Json<SendMyChurchInvitationRequest>,
) -> StatusCode {
    StatusCode::CREATED
}

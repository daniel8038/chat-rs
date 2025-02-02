use axum::response::IntoResponse;

pub async fn signin_handler() -> impl IntoResponse {
    "auth"
}
pub async fn signup_handler() -> impl IntoResponse {
    "auth"
}

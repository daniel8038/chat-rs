use axum::response::IntoResponse;

pub async fn list_chat_handler() -> impl IntoResponse {
    "auth"
}
pub async fn create_chat_handler() -> impl IntoResponse {
    "create"
}
pub async fn update_chat_handler() -> impl IntoResponse {
    "update_chat_handler"
}
pub async fn delete_chat_handler() -> impl IntoResponse {
    "delete_chat_handler"
}

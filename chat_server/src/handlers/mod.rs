pub mod auth;
pub mod chat;
pub mod messages;

use axum::response::IntoResponse;

pub(crate) use auth::*;
pub(crate) use chat::*;
pub(crate) use messages::*;

pub async fn index_handler() -> impl IntoResponse {
    "index"
}

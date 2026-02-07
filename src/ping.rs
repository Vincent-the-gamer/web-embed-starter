use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    code: u32,
    message: String,
}

#[axum::debug_handler]
pub async fn ping(Json(payload): Json<Message>) -> Json<Message> {
    Json(payload)
}

use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct Payload {
    pub message: String
}


pub async fn mirror_json(Json(body): Json<Payload>) -> impl IntoResponse {
    Json(body)
}
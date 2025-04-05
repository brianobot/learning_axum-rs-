use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct PublicUserData {
    pub username: String,
    pub active: bool,
}

pub async fn submit_user_data(Json(user_data): Json<UserData>) -> impl IntoResponse {
    let public_user_data = PublicUserData {
        username: user_data.username,
        active: true,
    };
    (StatusCode::ACCEPTED, Json(public_user_data))
}

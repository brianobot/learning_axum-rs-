use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn increment_counter() -> Response {
    (StatusCode::CREATED, "Some created data".to_owned()).into_response()
}

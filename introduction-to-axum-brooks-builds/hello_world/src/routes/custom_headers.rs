use axum::{http::HeaderMap, response::IntoResponse};

pub async fn get_custom_headers(_headers: HeaderMap) -> impl IntoResponse {
    "Custom Headers"
}
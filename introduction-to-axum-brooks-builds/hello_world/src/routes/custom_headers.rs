use axum::Extension;
use axum::{http::HeaderMap, response::IntoResponse};

use crate::routes::SharedData;

pub async fn get_custom_headers(
    Extension(shared_data): Extension<SharedData>,
    _headers: HeaderMap,
) -> impl IntoResponse {
    shared_data.message
}

#[derive(Clone)]
pub struct HeaderMessage(pub String);

pub async fn get_middleware_custom_headers(
    Extension(message): Extension<HeaderMessage>,
) -> impl IntoResponse {
    message.0
}

use axum::response::IntoResponse;
use axum_extra::{TypedHeader, headers::ContentType};

pub async fn get_request_headers(
    TypedHeader(content_type): TypedHeader<ContentType>,
) -> impl IntoResponse {
    content_type.to_string()
}

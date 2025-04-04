use axum::response::IntoResponse;

pub async fn mirror(body: String) -> impl IntoResponse {
    body
}

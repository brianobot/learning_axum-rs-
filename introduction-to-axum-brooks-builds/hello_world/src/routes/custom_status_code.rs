use axum::http::StatusCode;

pub async fn get_always_error() -> Result<(), StatusCode> {
    Err(StatusCode::IM_A_TEAPOT)
}

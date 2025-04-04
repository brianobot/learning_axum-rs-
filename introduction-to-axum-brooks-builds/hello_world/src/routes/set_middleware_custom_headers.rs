use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};

pub async fn extract_message_from_headers(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("Running the middlware here: ✅");
    Ok(next.run(req).await)
}

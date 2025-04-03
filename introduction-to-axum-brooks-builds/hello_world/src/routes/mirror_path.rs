use axum::extract::Path;

use axum::response::IntoResponse;


pub async fn mirror_path(Path(path): Path<String>) -> impl IntoResponse {
    path
}
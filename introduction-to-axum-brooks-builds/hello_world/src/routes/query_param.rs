use std::collections::HashMap;
use axum::extract::Json;

use axum::{extract::Query, response::IntoResponse};

pub async fn get_query_params(Query(query): Query<HashMap<String, String>>) -> impl IntoResponse {
    Json(query)
}
use std::collections::HashMap;

use axum::Json;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct Data {
    prev: Option<String>,
    next: Option<String>,
    count: i32,
    result: Vec<HashMap<String, String>>,
}

pub async fn get_json_response() -> Json<Data> {
    let data = Data::default();
    Json(data)
}

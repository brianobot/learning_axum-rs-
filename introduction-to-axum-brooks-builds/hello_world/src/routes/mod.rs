mod hello_world;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_path;
mod query_param;
mod request_headers;
mod custom_headers;

use axum::Router;
use axum::routing::{get, post};

use hello_world::hello_world;
use mirror_body_json::mirror_json;
use mirror_body_string::mirror;
use mirror_path::mirror_path;
use query_param::get_query_params;
use request_headers::get_request_headers;
use custom_headers::get_custom_headers;


pub fn create_router() -> Router {
    Router::new()
        .route("/", get(hello_world)) // a reference to the function is used here, not the called function
        .route("/", post(hello_world)) // a reference to the function is used here, not the called function
        .route("/mirror", post(mirror))
        .route("/mirror_json", post(mirror_json))
        .route("/mirror_path/{name}", post(mirror_path))
        .route("/query_param", get(get_query_params))
        .route("/request_headers", get(get_request_headers))
        .route("/custom_headers", get(get_custom_headers))
}

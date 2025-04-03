mod hello_world;
mod mirror_body_string;
mod mirror_body_json;
mod mirror_path;

use axum::Router;
use axum::routing::{get, post};

use hello_world::hello_world;
use mirror_body_string::mirror;
use mirror_body_json::mirror_json;
use mirror_path::mirror_path;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(hello_world)) // a reference to the function is used here, not the called function
        .route("/", post(hello_world)) // a reference to the function is used here, not the called function
        .route("/mirror", post(mirror))
        .route("/mirror_json", post(mirror_json))
        .route("/mirror_path/{name}", post(mirror_path))
}

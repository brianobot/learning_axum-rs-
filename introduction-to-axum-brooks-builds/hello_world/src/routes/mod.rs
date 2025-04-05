mod custom_headers;
mod custom_status_code;
mod custom_success_status_code;
mod hello_world;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_path;
mod query_param;
mod request_headers;
mod set_middleware_custom_headers;

use axum::routing::{get, post};
use axum::{Extension, Router, http, middleware};

use custom_headers::{HeaderMessage, get_custom_headers, get_middleware_custom_headers};
use custom_status_code::get_always_error;
use custom_success_status_code::increment_counter;
use hello_world::hello_world;
use mirror_body_json::mirror_json;
use mirror_body_string::mirror;
use mirror_path::mirror_path;
use query_param::get_query_params;
use request_headers::get_request_headers;
use set_middleware_custom_headers::extract_message_from_headers;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_router() -> Router {
    let cors = CorsLayer::new()
        .allow_methods(http::Method::POST)
        .allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from shared data".to_owned(),
    };

    Router::new()
        .route("/", get(hello_world)) // a reference to the function is used here, not the called function
        .route("/", post(hello_world)) // a reference to the function is used here, not the called function
        .route("/mirror", post(mirror))
        .route("/mirror_json", post(mirror_json))
        .route("/mirror_path/{name}", post(mirror_path))
        .route("/query_param", get(get_query_params))
        .route("/request_headers", get(get_request_headers))
        .route("/custom_headers", get(get_custom_headers))
        .layer(Extension(shared_data))
        .layer(cors) // the layers are applied to all the routes above it
        .route(
            "/middleware_custom_headers",
            get(get_middleware_custom_headers),
        )
        .layer(Extension(HeaderMessage(
            "Message injected into the header".to_owned(),
        )))
        .layer(middleware::from_fn(extract_message_from_headers))
        .route("/always_error", get(get_always_error))
        .route("/increment_counter", post(increment_counter))
}

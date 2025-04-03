mod hello_world;

use axum::Router;
use axum::routing::{get, post};

use hello_world::hello_world;


pub fn create_router() -> Router {
    Router::new()
        .route("/", get(hello_world)) // a reference to the function is used here, not the called function
        .route("/", post(hello_world)) // a reference to the function is used here, not the called function
}

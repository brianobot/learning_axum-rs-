use axum::response::Json;
use axum::{Router, routing::get};

use serde_json::{Value, json};
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().compact().init();

    let app = Router::<()>::new()
        .route("/", get(hello))
        // each route has some number of functions that relate to a http request verb
        // for multiple verbs chain those functions with the dot notation on the previous supported verbs
        // route("/blogs", get(blog).post(blog))
        .route("/bye", get(bye))
        .route("/json", get(json_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// whatever our handler function produces as output needs to implement the IntoResponse Trait
async fn hello() -> &'static str {
    "Hello world!"
}

async fn bye() -> String {
    String::from("Bye")
}

async fn json_handler() -> Json<Value> {
    info!("handling json request");
    Json(json!({"message": "hello internet"}))
}

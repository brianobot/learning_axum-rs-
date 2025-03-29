use std::collections::HashMap;

use axum::{Router, routing::get};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::<()>::new()
        .fallback(fallback)
        .route("/", get(hello))
        .route("/hello_uri", get(hello_uri))
        .route("/hello.html", get(hello_handler))
        .route("/hello_status", get(hello_status))
        .route("/demo.png", get(get_demo_png))
        .route("/demo.html", get(html_handler))
        .route("/demo_json", get(get_demo_json).put(put_demo_json))
        .route("/items", get(get_items))
        .route("/items/{id}", get(extract_path_parameter));

    // Run our application as a hyper server on http://localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> String {
    "Hello World!".into()
}

async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

// respond with html file
async fn hello_handler() -> axum::response::Html<&'static str> {
    include_str!("hello.html").into()
}

// respond with string
async fn html_handler() -> axum::response::Html<&'static str> {
    "<H1>Hello world!</H1> <br/><h2>Welcome to my Webpage</h2>".into()
}

// respond with status code
async fn hello_status() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::CREATED, "Everything is Ok".into())
}

// respond with the uri in the response
/*
here the arguments passed to the handler function is examined by axum
and if axum knows the type, it matches it to known extractors
*/
async fn hello_uri(uri: axum::http::Uri) -> String {
    format!("Uri is {}", uri)
}

async fn get_demo_png() -> impl axum::response::IntoResponse {
    use base64::Engine;
    let png = concat!(
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB",
        "CAYAAAAfFcSJAAAADUlEQVR42mPk+89Q",
        "DwADvgGOSHzRgAAAAABJRU5ErkJggg=="
    );

    (
        axum::response::AppendHeaders([(axum::http::header::CONTENT_TYPE, "image/png")]),
        base64::engine::general_purpose::STANDARD
            .decode(png)
            .unwrap(),
    )
}

async fn extract_path_parameter(axum::extract::Path(id): axum::extract::Path<String>) -> String {
    format!("Path parameter is {}", id)
}

async fn get_items(
    axum::extract::Query(query): axum::extract::Query<HashMap<String, String>>,
) -> String {
    format!("Query = {:?}", query)
}

async fn get_demo_json() -> axum::extract::Json<Value> {
    json!({"message": "Hello World!"}).into()
}

async fn put_demo_json(
    axum::extract::Json(data): axum::extract::Json<serde_json::Value>,
) -> axum::extract::Json<Value> {
    json!({"detail": "success", "data": data}).into()
}

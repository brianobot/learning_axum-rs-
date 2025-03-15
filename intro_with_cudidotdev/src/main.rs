use axum::{http::StatusCode, routing::get, Router};


#[tokio::main]
async fn main() {
    let app = Router::<()>::new()
        .route("/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn hello() -> (StatusCode, String) {
    (StatusCode::CREATED, format!("Hello World!"))
}
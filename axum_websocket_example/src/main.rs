use axum::routing::get;
use axum::{response::IntoResponse, Router};



#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started at http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}


async fn home() -> impl IntoResponse {
    "Home"
}
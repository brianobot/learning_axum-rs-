use axum::{routing::get, Router};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world)); // a reference to the function is used here, not the called function

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server started on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}


async fn hello_world() -> String {
    String::from("Hello World!")
}
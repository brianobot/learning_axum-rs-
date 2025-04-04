mod routes;

use routes::create_router;

pub async fn run() {
    let app = create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server started on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

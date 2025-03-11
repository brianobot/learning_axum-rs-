use axum::{routing::get, Router};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/bye", get(bye))
        .route("/profile", get(|| async { "Profile Page!" }))
        .route("/info", get(|| async { "Info Page!" }))
        .route("/contact-us", get(|| async { "Contact Us Page!" }))
        .route("/about", get(|| async { "About Page!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello world!"
}

async fn bye() -> String {
    String::from("Bye")
}
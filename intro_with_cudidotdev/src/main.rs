use axum::{body::Body, http::{Response, StatusCode}, response::IntoResponse, routing::{get, post}, Json, Router};
use serde::Serialize;


#[tokio::main]
async fn main() {
    let app = Router::<()>::new()
        .route("/hello", get(hello))
        .route("/hello", post(hello2));
    // routes are not allowed to have the same path for the same request verb
    // the above is allowed, but two path with the same request method cause the program to panic

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is listening on localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Debug)]
struct User {
    email: String
}

// when returning  a tuple with status code and string
// the status code must be the first item in the tuple
async fn hello() -> (StatusCode, Json<User>) {
    (StatusCode::CREATED, Json(User { email: "brianobot9@gmail.com".into() }))
}


// here we have more fine grain control over the creation of the response 
async fn hello2() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .header("X-Custom-Foo", "batman")
        .body(Body::from("Created"))
        .unwrap()
}
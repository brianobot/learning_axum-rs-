use axum::Json;
use axum::Router;
use axum::routing::get;

use serde::Serialize;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler)); // the route methods consumes and return the Router struct instance

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server Started.. Listening on localhost:3000");
    
    // await returns Result mostly so unwrap must be called to Handle error correctly
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Message {
    message: String
}


async fn handler() -> Json<Message> {
    Json(
        Message {
            message: String::from("Hello World!"),
        }
    )     
}
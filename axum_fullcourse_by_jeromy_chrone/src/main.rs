use axum::{extract::{Path, Query}, http::StatusCode, middleware, response::{Html, IntoResponse, Response}, routing::get, Router};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

mod error;
mod web;

pub use self::error::{Error, Result};

type NormalResult<T, E> = std::result::Result<T, E>;


fn route_hello() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
}

fn route_static() -> Router {
    Router::new()
        // nest service is used to serve directories instead of regular Response
        .nest_service("/asset", ServeDir::new("./"))
}


// this here is a middleware, it takes a response and return a response
// TODO: issue with middleware not working Found the Solution to this
// the answer is in the ordering of the layer, its position
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} main response mapper ", "RES_MAPPER");

    println!();

    res
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .fallback(|| async { "Not Found!" })
        .route("/ok", get(ok_handler))
        .route("/plain", get(plain_handler))
        .route("/razzy/{id}", get(razzy_handler))
        .route("/hello/{name}", get(hello2_handler))
        .route("/complex", get(complex_handler))
        .fallback_service(route_static())
        .merge(route_hello()) // this allow us to merge routes to a route
        .merge(web::login_routes::routes())
        .layer(middleware::map_response(main_response_mapper));
        // the layers are applied from both to top
        // so if you want some routes to be affect by a layout
        // those routes must be defined above the layer
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 LISTENING on localhost:3000");
    axum::serve(listener, app).await.unwrap();
}


async fn ok_handler() -> NormalResult<(StatusCode, String), (StatusCode, String)> {
    // this returns a 200 response even when i expected an error response
    // this is because the wrapped type, String default to a 200 response
    // Err(format!("BAD REQUEST"));
    Err((StatusCode::BAD_REQUEST, String::from("Hello World!")))
}


async fn plain_handler() {
    ()
}

#[derive(Debug, Deserialize, Serialize)]
struct HelloParams {
    name: Option<String>,
}

// an Handler function is an async function that returns an implementation of IntoResponse
async fn hello_handler(Query(query): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - hello handler", "HANDLER");
    println!("->> {:<12?} - query", query);

    let name = query.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}</strong>" ))
}


async fn hello2_handler(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - hello2 handler", "HANDLER");
    
    Html(format!("Hello {name}"))
}

async fn razzy_handler(Path(id): Path<String>) -> impl IntoResponse {
    format!("Razzy: {id}")
}


async fn complex_handler() -> (StatusCode, String) {
    (StatusCode::CONFLICT, format!("Conflict"))
}
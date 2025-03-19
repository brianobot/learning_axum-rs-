#![allow(unused)]
use std::sync::RwLock;
use std::sync::Arc;

use axum::extract::State;
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }).post(|| async { "Post Response ✉️" }))
        .route("/greet/", get(greet_query))
        .route("/greet/body", post(greet_body))
        .route("/greet/{name}", get(greet_path))
        .nest("/kingkong/", king())
        .merge(ping_pong())
        .merge(numbers_management()
    ); // there is no conflict even when the nested path endpoint ends with /kingkong/
        // and the nested path start with /
        // merging route adds the router to the current router
        // nesting route nested a router at a particular endpoint .nest("top_level/", router)

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server is listening on localhost:3000");

    axum::serve(listener, app).await.unwrap();
}


fn ping_pong() -> Router {
    // note, path must start with /
    Router::new().route("/ping", get(|| async { "pong!" }))
}


fn king() -> Router {
    async fn kong() -> impl IntoResponse {
        "Kong"
    }

    Router::new().route("/king", get(kong))
}

// Path is a tuple struct that wrap a generic value T
async fn greet_path(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello {name}!")
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryParameters {
    page: Option<u8>,
    page_size: Option<u8>,
}


async fn greet_query(Query(params): Query<QueryParameters>) -> impl IntoResponse {
    
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);


    format!("page: {page}, page_size: {page_size}")
}


// Json is both an extractor and a responder 
async fn greet_body(Json(params): Json<QueryParameters>) -> impl IntoResponse {
    println!("{params:?}");
    "".into_response()
}

// in a case where you have a branch in an handler functionthat return different
// types for each branch, you can call the into_response (which returns a Response type) on each of the returned
// value to ensure the return value are of the same type and the compiler does not complain


// implementing shared state in axum
#[derive(Default, Clone)]
struct AppState {
    numbers: Vec<i32>
}

type AppStateType = Arc<RwLock<AppState>>;
// we need an Arc of a RwLock of an AppState

// RW Lock: Reader.Writer Lock, make this thread
// Arc: Atomic Reference Counter

// Arc are always needed for states in axum

fn numbers_management() -> Router {
    // State is an extractor that allows us to extract shared state
    async fn get_numbers(State(state): State<AppStateType>) -> impl IntoResponse {
        // let value = state.read().await.numbers.clone();
        Json({})
    }

    async fn add_number(State(state): State<AppStateType>, Json(new_number): Json<i32>) -> impl IntoResponse {
        // let mut writable_state = state.write().await;
        // writable_state.numbers.push(new_number);
        // let value = writable_state.numbers.clone();
        Json({})
    }

    Router::new()
        .route("/numbers", get(get_numbers).post(add_number))
        .with_state(Arc::new(RwLock::new(AppState::default())))
}
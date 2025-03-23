use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};

pub mod logic;
pub use logic::*;


#[allow(unused)]
#[derive(Clone, Debug)]
struct AppState {
    name: String
}


#[tokio::main]
async fn main() {
    let app_state = AppState {
        name: "Test App".to_string()
    };

    let app = Router::new()
        .route("/", get(handler))
        .route("/home", get(home))
        .merge(inner_route())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}


fn inner_route() -> Router<AppState> {
    // when using inner routes they must specify the state type to compose with the outer router
    Router::new()
        .route("/inner",  get(|State(state): State<AppState>| async move { 
            println!("Inner Route State {:?}", state); 
            "Inner route" 
        }))
}


async fn handler(State(state): State<AppState>) -> impl IntoResponse {
    println!("state: {:?}", state);
    (StatusCode::ACCEPTED, "Hello World!")
}


async fn home(State(state): State<AppState>) -> impl IntoResponse {
    println!("app state: {:?}", state);
    "Home Endpoint"
}
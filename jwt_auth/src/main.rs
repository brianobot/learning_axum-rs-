use axum::{
    Json, Router,
    http::{Response, StatusCode, header},
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::json;

mod auth_utils;
use auth_utils::{User, get_jwt};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/public-view", get(public_view))
        .route("/get-token", post(get_token))
        .route("/secret-view", get(secret_view));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("✅ Server Started @ localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn public_view() -> impl IntoResponse {
    // the builder method return a Builder struct
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        // the body method must be called last before unwrapping the result
        // because the body method does not return a Builder it returns a result
        // a work around this is to call the other methods after calling unwrap on
        // the return type of the body method
        .body(
            json!(
                {
                    "success": true,
                    "data": {
                        "message": "This data is available to all users",
                    }
                }
            )
            .to_string(),
        )
        .unwrap()
}

async fn get_token(Json(user): Json<User>) -> impl IntoResponse {
    let token = get_jwt(user);

    let response = match token {
        Ok(token) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(json!({"success": true, "data": {"token": token}}).to_string())
            .unwrap_or_default(),

        Err(_) => Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header(header::CONTENT_TYPE, "application/json")
            .body(json!({"success": false, "data": {"message": "unauthorized"}}).to_string())
            .unwrap_or_default(),
    };

    response
}

async fn secret_view() -> impl IntoResponse {}

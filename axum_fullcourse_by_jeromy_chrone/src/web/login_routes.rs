use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{Error, Result};
use crate::web;

#[derive(Debug, Deserialize)] 
// because it has to be deserialized from json to rust
struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/api_login", post(api_login))
}

// below the Json type is a body extractor and there can only be one body extractor per handler
async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    // the reason we can use our result type, is that the error variant
    // implements the IntoResponse trait too, 
    println!("->> {:<12} -api_login ", "HANDLER");

    if payload.username == "brian" && payload.password == "testpassword" {
        let body = Json(json!({
            "result": {
                "success": true
            }
        }));

        cookies.add(Cookie::new(web::AUTH_TOKEN, "user1-.exp.sign"));

        Ok(body)
    } else {
        Err(Error::LoginFail)
    }
}
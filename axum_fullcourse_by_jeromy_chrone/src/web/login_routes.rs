use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{Error, Result};


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
async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    // the reason we can use our result type, is that the error variant
    // implements the IntoResponse trait too, 
    println!("->> {:<12} -api_login ", "HANDLER");

    if payload.username == "brian" && payload.password == "testpassword" {
        let body = Json(json!({
            "result": {
                "success": true
            }
        }));

        Ok(body)
    } else {
        Err(Error::LoginFail)
    }
}
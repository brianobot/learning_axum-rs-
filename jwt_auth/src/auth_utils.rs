use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header},
    response::Response,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;

use async_trait::async_trait;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub email: String,
}

pub struct Auth(User);

// #[async_trait]
// impl<S> FromRequestParts<S> for Auth
// where
//     S: Send + Sync,
// {
//     type Rejection = Response<String>;

//     async fn from_request_parts(
//         parts: &mut axum::http::request::Parts,
//         _: &S,
//     ) -> Result<Self, Self::Rejection> {
//         let access_token = parts
//             .headers
//             .get(header::AUTHORIZATION)
//             .and_then(|value| value.to_str().ok())
//             .and_then(|str| str.split(" ").nth(1));

//         match access_token {
//             Some(token) => {
//                 let user_data = decode_jwt(token);

//                 match user_data {
//                     Ok(user) => Ok(Auth(user)),
//                     Err(e) => Err(Response::builder()
//                         .status(StatusCode::UNAUTHORIZED)
//                         .header(header::CONTENT_TYPE, "application/json")
//                         .body(
//                             json!({"success": false, "data": {"message": e.to_string()}})
//                                 .to_string(),
//                         )
//                         .unwrap_or_default()),
//                 }
//             }
//             None => Err(Response::builder()
//                 .status(StatusCode::UNAUTHORIZED)
//                 .header(header::CONTENT_TYPE, "application/json")
//                 .body(
//                     json!({"success": false, "data": {"message": "no token provided"}}).to_string(),
//                 )
//                 .unwrap_or_default()),
//         }
//     }
// }

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub email: String,
    pub exp: i64,
}

pub fn get_jwt(user: User) -> Result<String, String> {
    let token = encode(
        &Header::default(),
        &Claims {
            email: user.email,
            exp: (Utc::now() + Duration::minutes(1)).timestamp(),
        },
        &EncodingKey::from_secret("mykey".as_bytes()),
    )
    .map_err(|e| e.to_string());

    token
}

pub fn decode_jwt(token: &str) -> Result<User, String> {
    let token_data = decode::<User>(
        token,
        &DecodingKey::from_secret("mykey".as_bytes()),
        &Validation::default(),
    );

    match token_data {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => Err(e.to_string()),
    }
}

use axum::{
    Json,
    extract::FromRequestParts,
    http::{HeaderValue, StatusCode, header::USER_AGENT, request::Parts},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

pub struct CustomExtractor(HeaderValue);

impl<S> FromRequestParts<S> for CustomExtractor
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        if let Some(user_agent) = parts.headers.get(USER_AGENT) {
            Ok(CustomExtractor(user_agent.clone()))
        } else {
            Err((StatusCode::BAD_REQUEST, "`User-Agent` header is missing"))
        }
    }
}

pub async fn custom_json_extractor(
    CustomExtractor(user_agent): CustomExtractor,
    Json(auth_data): Json<AuthData>,
) -> (StatusCode, String) {
    dbg!(auth_data);
    dbg!(user_agent);
    (StatusCode::ACCEPTED, "Processed auth data".to_owned())
}

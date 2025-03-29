use serde::{Deserialize, Serialize};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{Level, info};

use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Please add an .env file to the project root");

    tracing_subscriber::fmt().init();

    let database_url = std::env::var("DATABASE_URL").expect("Please set the database url env");

    let db_pool = PgPoolOptions::new()
        .max_connections(5) // when using this, this must be provided before the connect
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database!");

    let app = Router::new()
        .route("/", get(|| async { "Home" }))
        .route("/users", get(list_users))
        .route("/users/", get(list_users))
        .with_state(db_pool)
        .layer::<L>(TraceLayer::new_for_http().into());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to Connect to addresss");
    info!("🚀 Server listening on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("Fail to start serve server");
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
struct User {
    id: String,
    email: String,
    avatar: Option<String>, // fields that can be null or empty must be wrapped in Option type
    username: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct UsernameQuery {
    username: Option<String>,
}

async fn list_users(
    State(pool): State<Pool<Postgres>>,
    Query(query): Query<UsernameQuery>,
) -> (StatusCode, impl IntoResponse) {
    let users = sqlx::query_as!(User, "SELECT id, email, avatar, username FROM users_user")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch users");

    (StatusCode::OK, Json(users))
}

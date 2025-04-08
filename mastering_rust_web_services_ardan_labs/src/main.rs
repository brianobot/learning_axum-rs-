mod db;

use crate::db::{all_books, get_book, init_db};
use anyhow::Result;
use axum::{Router, routing::get};
use dotenvy;

#[tokio::main]
async fn main() -> Result<()> {
    // load the .env
    dotenvy::dotenv().ok();

    let connection_pool = init_db().await?;

    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/books", get(all_books))
        .route("/books/{id}", get(get_book))
        .with_state(connection_pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Server started @ localhost:3000");
    axum::serve(listener, app).await?;

    Ok(())
}

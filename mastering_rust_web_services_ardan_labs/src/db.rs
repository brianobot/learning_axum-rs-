use std::sync::RwLock;

use anyhow::Ok;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::Response;
use axum::response::IntoResponse;
use axum::{Json, debug_handler};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

pub async fn init_db() -> Result<SqlitePool, anyhow::Error> {
    let database_url = std::env::var("DATABASE_URL")?;
    let connection_pool = SqlitePool::connect(&database_url).await.unwrap();
    sqlx::migrate!().run(&connection_pool).await.unwrap();
    println!("Database migration applied ✅");
    Ok(connection_pool)
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
}

impl IntoResponse for Book {
    fn into_response(self) -> axum::response::Response {
        Response::new(Body::new("Hello".to_owned()))
    }
}

struct BookCache {
    all_books: RwLock<Option<Vec<Book>>>,
}

#[debug_handler]
pub async fn all_books(State(connection_pool): State<SqlitePool>) -> Json<Vec<Book>> {
    Ok(
        // here the turbo fish is used to annotate taht the type return is two types
        // the first is the type returned from the db, which we don't need
        // the second is the type we want to coerce it into
        Json(
            sqlx::query_as::<_, Book>("SELECT * FROM books ORDER BY title, author")
                .fetch_all(&connection_pool)
                .await
                .unwrap(),
        ),
    )
    .unwrap()
}

#[debug_handler]
pub async fn get_book(
    Path(id): Path<i32>,
    State(connection_pool): State<SqlitePool>,
) -> Json<Book> {
    Ok(Json(
        sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id=$1")
            .bind(id)
            .fetch_one(&connection_pool)
            .await
            .unwrap(),
    ))
    .unwrap()
}

// TODO: figure out why the test are failing
#[cfg(test)]
mod test {
    use super::*;

    #[sqlx::test]
    async fn test_init_db() {
        dotenvy::dotenv().unwrap();

        init_db().await.unwrap();
        let database_url = std::env::var("DATABASE_URL").unwrap();
        dbg!(&database_url);
        let connection_pool: sqlx::Pool<sqlx::Sqlite> =
            SqlitePool::connect(&database_url).await.unwrap();
        dbg!(&connection_pool);
        let json_books = all_books(State(connection_pool)).await;

        println!("Json Books: {json_books:?}");
    }
}

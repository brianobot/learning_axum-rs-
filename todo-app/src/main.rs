use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, migrate::MigrateDatabase, prelude::FromRow};

use log::info;
use tower_http::trace::TraceLayer;
// use tracing_subscriber::EnvFilter;

const DB_URL: &str = "sqlite://sqlite.db";

#[tokio::main]
async fn main() {
    // 1. Initialize tracing + log bridging
    tracing_subscriber::fmt()
        // This allows you to use, e.g., `RUST_LOG=info` or `RUST_LOG=debug`
        // when running the app to set log levels.
        // .with_env_filter(
        //     EnvFilter::try_from_default_env()
        //         .or_else(|_| EnvFilter::try_new("todo-app=debug,tower_http=debug"))
        //         .unwrap(),
        // )
        .init();

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("ℹ️ Creating Database {DB_URL}");
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => info!("✅ Created Database successfully"),
            Err(error) => panic!("Error creating data: {error}"),
        }
    } else {
        info!("✅ Database Already exists");
    }

    let pool = sqlx::sqlite::SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    info!("✅ Database Migration Applied");

    let app = Router::new()
        .route("/", get(home))
        .route("/todos", get(list_todos).post(create_todos))
        .route("/todos/", get(list_todos).post(create_todos))
        .route(
            "/todos/{todo_id}",
            get(get_todo).patch(update_todo).delete(delete_todo),
        )
        .route(
            "/todos/{todo_id}/",
            get(get_todo).patch(update_todo).delete(delete_todo),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("🚀 Server is Running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, FromRow, Serialize)]
struct Todo {
    id: i32,
    name: String,
    description: Option<String>,
}

async fn home() -> impl IntoResponse {
    "Home"
}

async fn list_todos(State(pool): State<Pool<Sqlite>>) -> impl IntoResponse {
    let todos: Vec<Todo> = sqlx::query_as("SELECT id, name, description FROM todos")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(todos)
}

#[derive(Serialize, Deserialize)]
struct CreateTodo {
    name: String,
    description: Option<String>,
}

async fn create_todos(
    State(pool): State<Pool<Sqlite>>,
    Json(payload): Json<CreateTodo>,
) -> (StatusCode, impl IntoResponse) {
    let query_result = sqlx::query("INSERT INTO todos (name, description) VALUES ($1, $2)")
        .bind(payload.name.clone())
        .bind(payload.description.clone())
        .execute(&pool)
        .await
        .expect("Failed to Create Todo Record");

    println!("Rows Affected: {}", query_result.rows_affected());
    (StatusCode::CREATED, Json(payload))
}

async fn get_todo(Path(todo_id): Path<i32>, State(pool): State<Pool<Sqlite>>) -> impl IntoResponse {
    let todo: Todo = sqlx::query_as("SELECT * FROM todos where id = $1")
        .bind(todo_id)
        .fetch_one(&pool)
        .await
        .expect("Failed to get todo with id");

    Json(todo)
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
struct UpdateTodo {
    name: Option<String>,
    description: Option<String>,
}

async fn update_todo(
    Path(todo_id): Path<i32>,
    State(pool): State<Pool<Sqlite>>,
    Json(payload): Json<UpdateTodo>,
) -> impl IntoResponse {
    let todo: UpdateTodo = sqlx::query_as("SELECT * FROM todos where id = $1")
        .bind(todo_id)
        .fetch_one(&pool)
        .await
        .expect("Failed to get todo with id");

    sqlx::query("UPDATE todos SET name = $1, description = $2 where id = $3")
        .bind(payload.name.unwrap_or(todo.name.unwrap()))
        .bind(payload.description.unwrap_or(todo.description.unwrap()))
        .bind(todo_id)
        .execute(&pool)
        .await
        .expect("Failed to get todo with id");

    StatusCode::ACCEPTED
}

async fn delete_todo(
    Path(todo_id): Path<i32>,
    State(pool): State<Pool<Sqlite>>,
) -> impl IntoResponse {
    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(todo_id)
        .execute(&pool)
        .await
        .expect("Failed to get todo with id");

    StatusCode::NO_CONTENT
}

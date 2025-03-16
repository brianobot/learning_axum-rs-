use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{delete, get, patch, post}, Router};

use serde::Serialize;
use serde_json::json;
use sqlx::{pool::PoolOptions, postgres::PgPoolOptions, PgPool};


// this is the struc that out sqlx query would be cast to
#[derive(Serialize)]
struct TaskRow {
    task_id: i32,
    name: String,
    priority: Option<i32>
}


#[tokio::main]
async fn main() {
    // expose environment variables from the .env file
    dotenv::dotenv().expect("Please set the env first");
    // after this the variable can be access from the std::env::var function

    // set envs
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".into());
    let database_url = std::env::var("DATABASE_URL").expect("Please provide the database url variable");

    println!("🌐 Server Address: {server_address}");
    println!("🗂️ Database URL: {database_url}");
    println!();

    // create db pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await
        .expect("Can't connect to the database");

    // compose  route 
    let app = Router::new()
        .route("/", get(|| async {"Hello World!"}))
        .route("/tasks", get(list_tasks))
        .route("/tasks", post(create_task))
        .route("/tasks/{task_id}", get(get_task))
        .route("/tasks/{task_id}", patch(update_task))
        .route("/tasks/{task_id}", delete(delete_task))
        .with_state(db_pool);

    // create TCP listener
    let listener = tokio::net::TcpListener::bind(server_address.clone()).await.unwrap();
    println!("🚀 Server listening on {}", server_address);

    // serve the application
    axum::serve(listener, app).await.unwrap();
}



async fn list_tasks(State(pg_pool): State<PgPool>) -> impl IntoResponse {
    // let rows = sqlx::query_as!(TaskRow, "SELECT * FROM tasks ORDER BY task_id")
    //     .fetch_all(&pg_pool)
    //     .await
    //     .map_err(|e| 
    //         (
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             json!({
    //                 "success": false,
    //                 "message": e.to_string()
    //             }).to_string(),
    //         )
    //     )?;

    // Ok(
    //     (
    //         StatusCode:: OK,
    //         json!({ "success": true, "data": rows }).to_string(),
    //     )
    // )

    format!("List Tasks: pg pool: {:?}", pg_pool)

}

async fn create_task(State(pg_pool): State<PgPool>) -> impl IntoResponse {
    "create task"
}

async fn get_task(Path(task_id): Path<String>, State(pg_pool): State<PgPool>) -> impl IntoResponse {
    format!("get task id: {task_id}")
}

async fn update_task(Path(task_id): Path<String>, State(pg_pool): State<PgPool>) -> impl IntoResponse {
    "update task:"
}

async fn delete_task(Path(task_id): Path<String>, State(pg_pool): State<PgPool>) -> impl IntoResponse {
    "delete task"
}

use axum::{
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
};

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, postgres::PgPoolOptions};

// this is the struc that out sqlx query would be cast to
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
struct TaskRow {
    task_id: i32,
    name: String,
    description: Option<String>,
    priority: Option<bool>,
}

#[tokio::main]
async fn main() {
    // exposes environment variables from the .env file
    dotenvy::dotenv().expect("Please add an .env file to the project root");
    // after this the variable can be access from the std::env::var function

    // set envs
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".into());
    let database_url =
        std::env::var("DATABASE_URL").expect("Please provide the database url variable");

    println!("🌐 Server Address: {server_address}");
    println!("🗂️ Database URL: {database_url}\n");

    // create db pool
    // NOTE: that db_pool is a wrapper round an Arc
    let db_pool = PgPoolOptions::new()
        .max_connections(5) // when using this, this must be provided before the connect
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database!");

    sqlx::migrate!()
        .run(&db_pool)
        .await
        .expect("Failed to run migrations!");

    println!("✅ Connected to Database!");

    // compose  route
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/tasks", get(list_tasks))
        .route("/tasks", post(create_task))
        .route("/tasks/{task_id}", get(get_task))
        .route("/tasks/{task_id}", patch(update_task))
        .route("/tasks/{task_id}", delete(delete_task))
        .with_state(db_pool);

    // create TCP listener
    let listener = tokio::net::TcpListener::bind(server_address.clone())
        .await
        .unwrap();
    println!("🚀 Server listening on {}", server_address);

    // serve the application
    axum::serve(listener, app).await.unwrap();
}

async fn list_tasks(State(pg_pool): State<PgPool>) -> (StatusCode, impl IntoResponse) {
    let _rows = sqlx::query_as!(TaskRow, "SELECT * FROM tasks ORDER BY task_id")
        .fetch_all(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({
                    "success": false,
                    "message": e.to_string()
                })
                .to_string(),
            )
        })
        .unwrap();

    println!("Rows: {:?}", _rows);

    (StatusCode::OK, Json(_rows))

    // Compile time checked queries are done with the query! versions of the query method
    // the difference is that the arguments to query! are passed after the query like the println! format
    // the advantage of the macro is that SQLx connects to the dev databs and lets the database itself vaerify the query with the values types provided

    // if for instance the table does not exist, the program would not compile
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateTask {
    name: String,
    description: Option<String>,
}

async fn create_task(
    State(pg_pool): State<PgPool>,
    Json(payload): Json<CreateTask>,
) -> (StatusCode, impl IntoResponse) {
    println!("payload: {:?}", payload);

    sqlx::query!(
        "INSERT INTO tasks (name, description) VALUES ($1, $2)",
        &payload.name,
        payload.description
    )
    .execute(&pg_pool)
    .await
    .unwrap();

    let task = sqlx::query_as!(
        TaskRow,
        "SELECT * FROM tasks WHERE name=$1 ORDER BY task_id DESC LIMIT 1",
        &payload.name
    )
    .fetch_one(&pg_pool)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(task))
}

async fn get_task(Path(task_id): Path<i32>, State(pg_pool): State<PgPool>) -> impl IntoResponse {
    let task = sqlx::query_as!(
        TaskRow,
        "SELECT * FROM tasks WHERE task_id=$1",
        task_id.clone()
    )
    .fetch_one(&pg_pool)
    .await
    .unwrap();

    Json(task)
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateTask {
    name: Option<String>,
    description: Option<String>,
    priority: Option<bool>,
}

async fn update_task(
    Path(task_id): Path<i32>,
    State(pg_pool): State<PgPool>,
    Json(payload): Json<UpdateTask>,
) -> (StatusCode, impl IntoResponse) {
    sqlx::query!(
        "UPDATE tasks SET name=$1, description=$2, priority=$3 WHERE task_id=$4",
        payload.name,
        payload.description,
        false,
        task_id
    )
    .execute(&pg_pool)
    .await
    .unwrap();

    let task = sqlx::query_as!(TaskRow, "SELECT * FROM tasks WHERE task_id=$1", task_id)
        .fetch_one(&pg_pool)
        .await
        .unwrap();

    (StatusCode::ACCEPTED, Json(task))
}

async fn delete_task(Path(task_id): Path<i32>, State(pg_pool): State<PgPool>) -> StatusCode {
    sqlx::query!("DELETE FROM tasks where task_id=$1", task_id)
        .execute(&pg_pool)
        .await
        .unwrap();

    StatusCode::NO_CONTENT
}

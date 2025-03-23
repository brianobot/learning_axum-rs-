use core::panic;

use sqlx::{migrate::MigrateDatabase, Sqlite};


const DB_URL: &str = "sqlite://sqlite.db";


#[tokio::main]
async fn main() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("ℹ️ Creating Database {DB_URL}");
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("✅ Created Database successfully"),
            Err(error) => panic!("Error creating data: {error}"),
        }
    } else {
        println!("✅ Database Already exists");
    }
}

# Mastering Rust Web Services: From Axum to CRUD Operations

## Web Service Layers
- Database layer: SQlite (in memory)
- Data Model: SQLx
- REST API: Axum
- Webview: Html, Javascript
- Deploy: Docker


# Steps
- Create Migrate with sqlx migrate add <migration_description>
- Edit Migration file to contain migration logic, e.g create db, populate db etc
- sqlx is tightly coupled with the database to be used, so it features and codes would reflect that database
- 
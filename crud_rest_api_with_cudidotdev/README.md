# Project Setup

Setup postgres server locally on Macos with this [link](https://www.youtube.com/watch?v=wTqosS71Dc4) 

## Create Postgres DB from the Command Line
```bash
sudo -u postgres psql
```

## Create User and Database

### With psql cli
```psql
> CREATE ROLE axum_postgres WITH PASSWORD 'axum_postgres';
> CREATE DATABASE 'axum_postgres' WITH OWNER = 'axum_postgres';
```

## Create Tables
```psql
> CREATE TABLE (
    task_id SERIAL PRIMARY KEY
    name VAR CHAR 
    priority INT
)
```

### With sqlx cli
[Read here](https://mo8it.com/blog/sqlx-interacting-with-databases-in-rust/)

1. Create database (Ensure the database url exist in the environment)
    ```bash
    sqlx database create
    ```

2. Create Migration
    ```bash
    sqlx migrate add create_tasks_table
    ```

3. Generate Build script to trigger recompilation when a new migration is added
    ```bash
    sqlx migrate build-script
    ```

4. Write Migration into the Migration script
    ```sql
    -- Add migration script here
    CREATE TABLE IF NOT EXISTS tasks (
        id SERIAL PRIMARY KEY,
        title TEXT NOT NULL,
        priority BOOLEAN NOT NULL DEFAULT false
    );
    ```

5. Apply Migration
    ```bash
    sqlx migrate run
    ```



## State 
State is a Way to share a Data Structure among handlers in an Axum project.
Since each handler runs in it own async context, the data structure must be wrapped in an Arc to avoid
duplicates when each handlers uses the state the data


### SQLX Query Structure
The sqlx query follows this structure

sqlx::<query_method/query_macro>
    .<optional_method bind only used with query methods not the macros> e.g bind
    .<executor>(&pool)
    .await
    .unwrap()

Examples of the executor are
- fetch, fetch_one, fetch_all, execute

Note
A struct doesn't have to derive sqlx::FromRow for using the macro query_as!.

### Embedded migration
use the following code to run migration automatically when the binary is ran
sqlx::migrate!()
    .run(&pool)
    .await
    .unwrap();


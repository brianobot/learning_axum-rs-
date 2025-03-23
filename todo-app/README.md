# TODO API


Setting Up Axum with Sqlite Tutorial [Link](https://tms-dev-blog.com/rust-sqlx-basics-with-sqlite/)


## Notes on Sqlite X Sqlx Combo
- For Some reason, the regular ```sqlx database create``` command does not work with sqlite, so i have to create
  the database in the code, 
- ```sqlx migrate add <migration_name>``` does work
- ```sqlx migrate run``` does not work, so i have to apply migration in the code before starting up the server
- using query!, query_as! gives some errors but using the reguar query does not give the same error
- the ```FromRow``` trait allows us to use the data structure in the query_as method or macro
- 
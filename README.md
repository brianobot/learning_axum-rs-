# Learning Axum by Brian Obot

This repository documents my journey of learning Axum, a web framework built with Rust that focuses on ergonomics and performance. The repo contains various examples, experiments, and notes as I explore Axum's features and capabilities.

## Why Axum?

Axum is a modern web framework built on top of Tokio and Hyper, making it well-suited for asynchronous applications. Key benefits include:

- Ergonomic request handling with typed extractors.
- Integration with Tokio for async performance.
- Modular architecture supporting middleware and routing flexibility.
- Strong typing and safety using Rust's ownership model.


## What You'll Find Here

This repo is structured with examples and notes that cover:

- Basic Axum setup and routing.
- Handling requests and responses.
- Extractors and middleware.
- Error handling and logging.
- Async database interactions with SQL/NoSQL.
- Authentication and authorization patterns.
- WebSockets, streaming, and real-time features.

Running the Examples

To run any example, ensure you have Rust installed (preferably via rustup), then clone the repository:

```bash
git clone https://github.com/brianobot/learning_axum-rs.git
cd learning_axum-rs
cargo run
```

Some examples may require additional dependencies. Refer to Cargo.toml for details.

## Contributions & Feedback

This is primarily a learning project, but feel free to open issues or PRs if you have suggestions or improvements!
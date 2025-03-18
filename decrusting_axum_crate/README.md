# Reference Youtube Video
[https://www.youtube.com/watch?v=Wnb_n5YktO8](https://www.youtube.com/watch?v=Wnb_n5YktO8)

Overview
Axum basically only provides routing and request handling
most of it other functionalities are provided by other crates like the tower crate etc.

Axum uses;
- hyper for handling all the http bits
- tower for handling everthing that has to do with middleware and the like
- tracing for anything that has to do with logs

Router is the primary entry point to an axum application,  

Panics inside handlers are isolated from each other, whereby a panic in a handler doesn't
affect the entire web application.


### Inventing the Service Trait
[Blog Post](https://tokio.rs/blog/2021-05-14-inventing-the-service-trait)


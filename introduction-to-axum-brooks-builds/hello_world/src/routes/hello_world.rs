// the handler does not need to know the http methods it is been called on
// the route is the person concerned with this information
pub async fn hello_world() -> String {
    String::from("Hello World!")
}

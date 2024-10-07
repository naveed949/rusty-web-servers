use tide::Server;
use crate::handler::{hello, post_hello, greet, json_response};
use crate::middleware::boxed_log_middleware;

pub fn configure_router() -> Server<()> {
    let mut app = tide::new();
    app.with(boxed_log_middleware); // Use the boxed middleware
    
    // Set up routes
    app.at("/").get(hello);
    app.at("/greet/:name").get(greet);
    app.at("/post").post(post_hello);
    app.at("/json").get(json_response);
    
    app
}
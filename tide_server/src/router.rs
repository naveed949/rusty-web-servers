use tide::Server;
use crate::handler::{hello, post_hello, greet, json_response, query_handler};
use crate::middleware::boxed_log_middleware;

pub fn configure_router() -> Server<()> {
    let mut app = tide::new();
    app.with(boxed_log_middleware); // Use the boxed middleware
    
    // Set up routes
    app.at("/").get(hello);
    app.at("/greet/:name").get(greet);
    app.at("/post").post(post_hello);
    app.at("/json").get(json_response);
    app.at("/query").get(query_handler);

    // nested routes under /v1 path
    app.at("/v1").nest({
        let mut nested = tide::new();
        nested.at("/hello").get(hello);
        nested.at("/greet/:name").get(greet);
        nested.at("/post").post(post_hello);
        nested.at("/json").get(json_response);
        nested.at("/query").get(query_handler);
        nested
    });
    
    app
}
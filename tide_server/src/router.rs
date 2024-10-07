use tide::Server;
use crate::handler::{get_cookie, greet, hello, json_response, not_found, post_hello, query_handler, remove_cookie, set_cookie};
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

    // serve static files
    app.at("/static").nest({
        let mut static_routes = tide::new();
        static_routes.at("/file").serve_file("static/text.txt").unwrap();
        static_routes.at("/").serve_dir("static/").unwrap();
        static_routes
    });
    
    // cookie handling
    app.at("/cookie").nest({
        let mut cookie_routes = tide::new();
        cookie_routes.at("/set").put(set_cookie);
        cookie_routes.at("/get").get(get_cookie);
        cookie_routes.at("/remove").get(remove_cookie);

        cookie_routes
    });

    // fallback route
    app.at("*").all(not_found);
    app
}
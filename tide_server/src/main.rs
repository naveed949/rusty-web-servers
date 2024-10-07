use tide::Request;
use tide::Response;
use tide::http::mime;
use tide::log;
use std::pin::Pin;
use std::future::Future;
use tide::{Next, Middleware, Error};

/// Handles GET requests to the root ("/") endpoint.
///
/// # Arguments
///
/// * `_req` - The incoming request (unused in this function).
///
/// # Returns
///
/// A `tide::Result` containing the string "Hello, world!".
async fn hello(_req: Request<()>) -> tide::Result {
    Ok("Hello, world!".into())
}

/// Represents the expected JSON structure for POST requests.
#[derive(serde::Deserialize)]
struct PostData {
    name: String,
    age: i32,
}

/// Handles POST requests to the "/post" endpoint.
///
/// # Arguments
///
/// * `req` - The incoming request containing JSON data.
///
/// # Returns
///
/// A `tide::Result` containing a greeting message based on the POSTed data.
async fn post_hello(mut req: Request<()>) -> tide::Result {
    let data: PostData = req.body_json().await?;
    Ok(format!("Hello, {}! You are {} years old.", data.name, data.age).into())
}

/// Middleware function for logging incoming requests and outgoing responses.
///
/// # Arguments
///
/// * `req` - The incoming request.
/// * `next` - The next middleware or route handler in the chain.
///
/// # Returns
///
/// A `Result` containing the `Response` or an `Error`.
async fn log_middleware<'a>(req: Request<()>, next: Next<'a, ()>) -> Result<Response, Error> {
    log::info!("Incoming request: {} {}", req.method(), req.url());
    let res = next.run(req).await;
    log::info!("Response status: {}", res.status());
    Ok(res)
}

/// Wraps the `log_middleware` function in a `Pin<Box<dyn Future>>`.
///
/// This function is necessary because Tide's `with` method expects
/// a middleware function that returns a `Pin<Box<dyn Future>>`.
///
/// # Arguments
///
/// * `req` - The incoming request.
/// * `next` - The next middleware or route handler in the chain.
///
/// # Returns
///
/// A `Pin<Box<dyn Future>>` containing the result of `log_middleware`.
fn boxed_log_middleware<'a>(req: Request<()>, next: Next<'a, ()>) -> Pin<Box<dyn Future<Output = Result<Response, Error>> + Send + 'a>> {
    Box::pin(log_middleware(req, next))
}

/// The main function that sets up and runs the Tide server.
#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start(); // Initialize the logger

    log::info!("Starting Tide Server");
    let mut app = tide::new();
    app.with(boxed_log_middleware); // Use the boxed middleware
    
    // Set up routes
    app.at("/").get(hello);
    app.at("/greet/:name").get(greet);
    app.at("/post").post(post_hello);
    app.at("/json").get(json_response);
    
    // Start the server
    app.listen("127.0.0.1:8080").await?;
    
    Ok(())
}

/// Handles GET requests to the "/greet/:name" endpoint.
///
/// # Arguments
///
/// * `req` - The incoming request containing the name parameter.
///
/// # Returns
///
/// A `tide::Result` containing a personalized greeting message.
async fn greet(req: Request<()>) -> tide::Result {
    let name: String = req.param("name")?.to_string();
    Ok(format!("Hello, {}!", name).into())
}

/// Handles GET requests to the "/json" endpoint.
///
/// # Arguments
///
/// * `_req` - The incoming request (unused in this function).
///
/// # Returns
///
/// A `tide::Result` containing a JSON response with a message and status.
async fn json_response(_req: Request<()>) -> tide::Result {
    let data = serde_json::json!({
        "message": "Hello, world!",
        "status": "success"
    });
    let mut res = Response::new(200);
    res.set_body(data.to_string());
    res.set_content_type(mime::JSON);
    Ok(res)
}

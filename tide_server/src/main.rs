use tide::Request;
use tide::Response;
use tide::http::mime;
use tide::log;
use std::pin::Pin;
use std::future::Future;
use tide::{Next, Middleware, Error};

async fn hello(_req: Request<()>) -> tide::Result {
    Ok("Hello, world!".into())
}

#[derive(serde::Deserialize)]
struct PostData {
    name: String,
    age: i32,
}

async fn post_hello(mut req: Request<()>) -> tide::Result {
    let data: PostData = req.body_json().await?;
    Ok(format!("Hello, {}! You are {} years old.", data.name, data.age).into())
}

async fn log_middleware<'a>(req: Request<()>, next: Next<'a, ()>) -> Result<Response, Error> {
    log::info!("Incoming request: {} {}", req.method(), req.url());
    let res = next.run(req).await;
    log::info!("Response status: {}", res.status());
    Ok(res)
}

fn boxed_log_middleware<'a>(req: Request<()>, next: Next<'a, ()>) -> Pin<Box<dyn Future<Output = Result<Response, Error>> + Send + 'a>> {
    Box::pin(log_middleware(req, next))
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start(); // Initialize the logger

    log::info!("Starting Tide Server");
    let mut app = tide::new();
    app.with(boxed_log_middleware); // Use the boxed middleware
    app.at("/").get(hello);
    app.at("/greet/:name").get(greet);
    app.at("/post").post(post_hello);
    app.at("/json").get(json_response);
    app.listen("127.0.0.1:8080").await?;
    
    Ok(())
}

async fn greet(req: Request<()>) -> tide::Result {
    let name: String = req.param("name")?.to_string();
    Ok(format!("Hello, {}!", name).into())
}

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

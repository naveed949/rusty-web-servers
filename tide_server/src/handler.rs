use tide::Request;
use tide::Response;
use tide::http::mime;
use tide::Result as TideResult;

#[derive(serde::Deserialize)]
struct PostData {
    name: String,
    age: i32,
}

pub async fn hello(_req: Request<()>) -> TideResult {
    Ok("Hello, world!".into())
}

pub async fn post_hello(mut req: Request<()>) -> TideResult {
    let data: PostData = req.body_json().await?;
    Ok(format!("Hello, {}! You are {} years old.", data.name, data.age).into())
}

pub async fn greet(req: Request<()>) -> TideResult {
    let name: String = req.param("name")?.to_string();
    Ok(format!("Hello, {}!", name).into())
}

pub async fn json_response(_req: Request<()>) -> TideResult {
    let data = serde_json::json!({
        "message": "Hello, world!",
        "status": "success"
    });
    let mut res = Response::new(200);
    res.set_body(data.to_string());
    res.set_content_type(mime::JSON);
    Ok(res)
}
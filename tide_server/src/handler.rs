use tide::Request;
use tide::Response;
use tide::http::mime;
use tide::Result as TideResult;

#[derive(serde::Deserialize)]
struct PostData {
    name: String,
    age: i32,
}
#[derive(serde::Deserialize)]
struct QueryParams {
    name: String,
    age: Option<i32>,
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

pub async fn query_handler(req: Request<()>) -> TideResult {
    let query: QueryParams = req.query()?;
    let age_message = if let Some(age) = query.age {
        format!(" You are {} years old.", age)
    } else {
        String::from("")
    };
    Ok(format!("Hello, {}!{}", query.name, age_message).into())
}
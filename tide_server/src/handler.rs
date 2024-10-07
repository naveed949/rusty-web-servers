use tide::log;
use tide::Request;
use tide::Response;
use tide::http::mime;
use tide::Result as TideResult;
use crate::repository::Model;
use crate::repository::SharedRepository;

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

// cookie handling
pub async fn set_cookie(_req: Request<()>) -> TideResult {
    let mut res = Response::new(200);
    res.insert_cookie(tide::http::Cookie::new("name", "tide"));
    Ok(res)
}

pub async fn get_cookie(req: Request<()>) -> TideResult {
    let cookie = req.cookie("name").unwrap();
    Ok(format!("Hello, {}!", cookie.value()).into())
}

pub async fn remove_cookie(_req: Request<()>) -> TideResult {
    let mut res = Response::new(200);
    res.remove_cookie(tide::http::Cookie::named("name"));
    Ok(res)
}

// 404 handler
pub async fn not_found(_req: Request<()>) -> TideResult {
    Ok("Not Found".into())
}

// state handling

pub async fn get_data(req: Request<SharedRepository>) -> TideResult {
    let repo = req.state();
    let repo = repo.lock().unwrap();
    let key = req.param("key")?;
    log::info!("Getting data for key: {}", key);
    if let Some(model) = repo.get(key) {
        Ok(model.value().into())
    } else {
        Ok("Not Found".into())
    }
}

pub async fn set_data(mut req: Request<SharedRepository>) -> TideResult {
    let data: Model = req.body_json().await?;
    let repo = req.state();
    let key = data.key();
    let mut repo = repo.lock().unwrap();
    repo.insert(key.to_string(), data.value().to_string());
    Ok("Data inserted".into())
}
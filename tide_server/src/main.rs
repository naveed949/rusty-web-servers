use tide::Request;

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

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(hello);
    app.at("/post").post(post_hello);
    println!("Tide_Server running on port 8080");
    app.listen("127.0.0.1:8080").await?;
    
    Ok(())
}

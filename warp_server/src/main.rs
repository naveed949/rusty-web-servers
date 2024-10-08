use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a filter that matches the path `/hello`
    let hello = warp::path!("hello")
        .map(|| warp::reply::html("Hello, Warp!"));

    // fallback to 404
    let fallback = warp::any()
        .map(|| warp::reply::with_status("Invalid Route!", warp::http::StatusCode::NOT_FOUND));

    // Combine the two filters with `or`
    let routes = hello.or(fallback);

    // Start the server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
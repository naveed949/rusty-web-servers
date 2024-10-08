use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a filter that matches the path `/hello`
    let hello = warp::path!("hello")
        .map(|| warp::reply::html("Hello, Warp!"));

    // Start the server
    warp::serve(hello)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
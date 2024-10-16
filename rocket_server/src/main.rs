#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::serde::{json::serde_json, json::Json};
use rocket::tokio::time::{sleep, Duration};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;

use rocket::tokio::task::spawn_blocking;
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/greet/<name..>")]
fn hello(name: PathBuf) -> String {
    // Accepts a single path segment or multiple segments as a single parameter. The .. is a special syntax that tells Rocket to capture all remaining segments. e.g. /greet/Alice/Bob/Charlie will capture ["Alice", "Bob", "Charlie"]. The captured segments are then concatenated into a single PathBuf.
    format!("Hello, {}!", name.into_os_string().into_string().unwrap())
}

// ignored segments are not captured
#[get("/greets/<name>/<_>/<city>")]
async fn ignored_segment(name: &str, city: &str) -> String {
    format!("Hello, {} from {}!", name, city)
}

#[derive(Deserialize, Serialize)]
struct Task {
    id: u32,
    name: String,
}

#[post("/task", data = "<task>")]
async fn create_task(task: Json<Task>) -> Json<Task> {
    sleep(Duration::from_secs(2)).await;
    task
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // This will block the current thread, but not the core thread.
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("static/text.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

// generate forwarding routes
#[get("/sum/<a>/<b>")]
fn sum1(a: i32, b: i32) -> String {
    format!("{} + {} = {}", a, b, a + b)
}
#[get("/sum/<a>/<b>", rank = 2)]
fn sum2(a: &str, b: &str) -> String {
    let a = a.chars().filter(|c| c.is_digit(10)).collect::<String>();
    let b = b.chars().filter(|c| c.is_digit(10)).collect::<String>();
    format!(
        "{} + {} = {}",
        a,
        b,
        a.parse::<i32>().unwrap() + b.parse::<i32>().unwrap()
    )
}

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index])
// }

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount(
            "/",
            routes![index, hello, create_task, blocking_task, ignored_segment],
        )
        .mount("/static", FileServer::from("static"))
        .mount("/forwarding", routes![sum1, sum2])
        .launch()
        .await?;

    Ok(())
}

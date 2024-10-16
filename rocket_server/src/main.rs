#[macro_use]
extern crate rocket;
use rocket::serde::{json::serde_json, json::Json};
use rocket::tokio::time::{sleep, Duration};
use serde::{Deserialize, Serialize};
use std::io;

use rocket::tokio::task::spawn_blocking;
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/greet/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
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

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index])
// }

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, hello, create_task, blocking_task])
        .launch()
        .await?;

    Ok(())
}

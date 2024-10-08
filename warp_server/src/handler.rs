use serde::{Deserialize, Serialize};
use warp::Filter;

pub fn hello() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("hello").map(|| warp::reply::html("Hello, Warp!"))
}

pub fn greet_with_path_variable(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("greet" / String).map(|name| warp::reply::html(format!("Hello, {}!", name)))
}

#[derive(Deserialize, Serialize)]
struct GreetRequest {
    name: String,
    age: u8,
}

pub fn greet_with_query_parameters(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("greet")
        .and(warp::get())
        .and(warp::query::<GreetRequest>())
        .map(|query: GreetRequest| {
            warp::reply::html(format!(
                "Hello, {}! You are {} years old.",
                query.name, query.age
            ))
        })
}

pub fn serve_static_files(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("static").and(warp::fs::dir("static"))
}

#[derive(Serialize)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}
pub fn todo_list_json() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path("todo").and(warp::get()).map(|| {
        let todos = vec![
            Todo {
                id: 1,
                title: "Buy milk".to_string(),
                completed: false,
            },
            Todo {
                id: 2,
                title: "Buy eggs".to_string(),
                completed: true,
            },
        ];

        warp::reply::json(&todos)
    })
}
// Fallback route
pub fn fallback() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(|| warp::reply::html("No Route Found"))
}

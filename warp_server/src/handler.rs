use serde::{Deserialize, Serialize};
use warp::Filter;

use crate::middleware::{self, with_auth, Unauthorized};

// Define the handler functions (filters in warp)

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

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}
pub fn todo_list_json() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path("todo")
        .and(warp::get())
        .and(warp::path::end())
        .map(|| {
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

pub fn todo_list_json_with_id(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todo" / u64).and(warp::get()).map(|id| {
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

        let todo = todos.into_iter().find(|todo| todo.id == id);

        match todo {
            Some(todo) => warp::reply::json(&todo),
            None => warp::reply::json(&"Todo not found"),
        }
    })
}

pub fn todo_list_set_json(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("todo")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::path::end())
        .and(with_auth().untuple_one())
        .map(|todo: Todo| {
            log::info!("Received todo: {:?}", todo);
            warp::reply::json(&todo)
        })
}
// Fallback route
pub fn fallback() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(|| warp::reply::html("No Route Found"))
}

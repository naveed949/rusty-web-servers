use log::{error, info};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::Filter;

use crate::{
    middleware::{self, with_auth, with_db, Unauthorized, User},
    repository::{InMemoryDB, Todo},
};

// Define the handler functions (filters in warp)

pub fn hello() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("hello").map(|| {
        info!("Received request for /hello");
        warp::reply::html("Hello, Warp!")
    })
}

pub fn greet_with_path_variable(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("greet" / String).map(|name| {
        info!("Received request for /greet/{}", name);
        warp::reply::html(format!("Hello, {}!", name))
    })
}

#[derive(Deserialize, Serialize, Debug)]
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
            info!(
                "Received request for /greet with query parameters: {:?}",
                query
            );
            warp::reply::html(format!(
                "Hello, {}! You are {} years old.",
                query.name, query.age
            ))
        })
}

pub fn serve_static_files(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    info!("Serving static files from /static");
    warp::path("static").and(warp::fs::dir("static"))
}

pub fn todo_list_json(
    db: InMemoryDB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("todo")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .map(|db_pool: InMemoryDB| {
            info!("Received request for /todo");
            let todos = db_pool.read_all_items();
            info!("Fetched todos: {:?}", todos);
            warp::reply::json(&todos)
        })
}

pub fn todo_list_json_with_id(
    db: InMemoryDB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("todo" / u64)
        .and(warp::get())
        .and(with_db(db.clone()))
        .map(|id: u64, db_pool: InMemoryDB| {
            info!("Received request for /todo/{}", id);
            let todo = db_pool.read_item(id);

            match todo {
                Some(todo) => {
                    info!("Found todo: {:?}", todo);
                    warp::reply::json(&todo)
                }
                None => {
                    error!("Todo with id {} not found", id);
                    warp::reply::json(&"Todo not found")
                }
            }
        })
}

#[derive(Deserialize, Serialize, Debug)]
struct TodoRequest {
    title: String,
    completed: bool,
}
pub fn todo_list_set_json(
    db: InMemoryDB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("todo")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::path::end())
        .and(with_auth())
        .and(middleware::with_db(db.clone()))
        .map(|todo: TodoRequest, user: User, db_pool: InMemoryDB| {
            info!("Received todo: {:?} from user: {}", todo, user.name);
            let item = db_pool.create_item(todo.title, todo.completed);
            info!("Created todo with id: {}", item.id);
            warp::reply::json(&format!("Created todo with id: {}", item.id))
        })
}
// Fallback route
pub fn fallback() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(|| {
        error!("No route found for the request");
        warp::reply::html("No Route Found")
    })
}

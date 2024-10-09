use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::Filter;

use crate::{
    middleware::{self, with_auth, with_db, Unauthorized, User},
    repository::{InMemoryDB, Todo},
};

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

pub fn todo_list_json(
    db: InMemoryDB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("todo")
        .and(warp::get())
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .map(|dbPool: InMemoryDB| {
            let todos = dbPool.read_all_items();

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
            let todo = db_pool.read_item(id);

            match todo {
                Some(todo) => warp::reply::json(&todo),
                None => warp::reply::json(&"Todo not found"),
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
            log::info!("Received todo: {:?} from user: {}", todo, user.name);
            db_pool.create_item(todo.title, todo.completed);
            warp::reply::json(&"Todo created")
        })
}
// Fallback route
pub fn fallback() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(|| warp::reply::html("No Route Found"))
}

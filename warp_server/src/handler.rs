use serde::{Deserialize, Serialize};
use warp::Filter;

pub fn hello() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("hello").map(|| warp::reply::html("Hello, Warp!"))
}

pub fn greet_with_path_variable() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("greet" / String).map(|name| warp::reply::html(format!("Hello, {}!", name)))
}

#[derive(Deserialize, Serialize)]
struct GreetRequest {
    name: String,
    age: u8,
}

pub fn greet_with_query_parameters() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("greet")
        .and(warp::get())
        .and(warp::query::<GreetRequest>())
        .map(|query: GreetRequest| {
            warp::reply::html(format!("Hello, {}! You are {} years old.", query.name, query.age))
        })
}

pub fn fallback() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(|| warp::reply::html("No Route Found"))
}
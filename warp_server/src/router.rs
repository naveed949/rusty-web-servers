use warp::Filter;
use crate::handler::{hello, greet_with_path_variable, greet_with_query_parameters, fallback};

pub fn configure_router() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    hello()
        .or(greet_with_path_variable())
        .or(greet_with_query_parameters())
        .or(fallback())
}
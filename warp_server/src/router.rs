use crate::handler::{fallback, greet_with_path_variable, greet_with_query_parameters, hello};
use warp::Filter;

pub fn configure_router() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    hello()
        .or(greet_with_path_variable())
        .or(greet_with_query_parameters())
        .or(fallback())
}

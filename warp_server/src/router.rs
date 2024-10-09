use crate::handler::{
    fallback, greet_with_path_variable, greet_with_query_parameters, handle_rejection, hello,
    serve_static_files, todo_list_json, todo_list_json_with_id, todo_list_set_json,
};
use warp::Filter;

pub fn configure_router() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    hello()
        .or(greet_with_path_variable())
        .or(greet_with_query_parameters())
        .or(serve_static_files())
        .or(todo_list_json())
        .or(todo_list_json_with_id())
        .or(todo_list_set_json())
        .or(fallback())
        .recover(handle_rejection)
}

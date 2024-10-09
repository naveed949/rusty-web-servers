use crate::errors::handle_rejection;
use crate::handler::{
    fallback, greet_with_path_variable, greet_with_query_parameters, hello, serve_static_files,
    todo_list_json, todo_list_json_with_id, todo_list_set_json,
};
use crate::middleware::with_db;
use crate::repository::InMemoryDB;
use warp::Filter;

pub fn configure_router(
    db: InMemoryDB,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // let db_filter = with_db(db);

    let hello_route = hello();
    let greet_with_path_variable_route = greet_with_path_variable();
    let greet_with_query_parameters_route = greet_with_query_parameters();
    let serve_static_files_route = serve_static_files();
    let todo_list_json_route = todo_list_json(db.clone());
    let todo_list_json_with_id_route = todo_list_json_with_id(db.clone());
    let todo_list_set_json_route = todo_list_set_json(db.clone());
    let fallback_route = fallback();

    hello_route
        .or(greet_with_path_variable_route)
        .or(greet_with_query_parameters_route)
        .or(serve_static_files_route)
        .or(todo_list_json_route)
        .or(todo_list_json_with_id_route)
        .or(todo_list_set_json_route)
        .or(fallback_route)
        .recover(handle_rejection)
        .boxed()
}

use warp::Filter;

pub fn with_auth() -> impl Filter<Extract = ((),), Error = warp::Rejection> + Clone {
    warp::header::optional::<String>("authorization").and_then(
        |auth_header: Option<String>| async move {
            match auth_header {
                Some(token) if token == "Bearer mysecrettoken" => Ok(()),
                _ => Err(warp::reject::custom(Unauthorized)),
            }
        },
    )
}

#[derive(Debug)]
pub struct Unauthorized;

impl warp::reject::Reject for Unauthorized {}

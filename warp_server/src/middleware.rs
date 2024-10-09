use warp::Filter;

pub fn with_auth() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    warp::header::optional::<String>("authorization").and_then(
        |auth_header: Option<String>| async move {
            match auth_header {
                Some(token) if token == "Bearer mysecrettoken" => Ok(User {
                    id: 1,
                    name: "Alice".to_string(),
                }),
                _ => Err(warp::reject::custom(Unauthorized)),
            }
        },
    )
}

#[derive(Debug)]
pub struct Unauthorized;

impl warp::reject::Reject for Unauthorized {}

pub struct User {
    pub id: u64,
    pub name: String,
}

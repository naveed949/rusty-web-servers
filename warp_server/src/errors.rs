use crate::middleware;

// Define the function to handle rejections
pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if err.is_not_found() {
        Ok(warp::reply::with_status(
            "No Route Found",
            warp::http::StatusCode::NOT_FOUND,
        ))
    } else if let Some(_) = err.find::<middleware::Unauthorized>() {
        Ok(warp::reply::with_status(
            "Unauthorized",
            warp::http::StatusCode::UNAUTHORIZED,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Internal Server Error",
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

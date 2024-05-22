use crate::handlers;
use warp::Filter;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_post()
        // Add other routes here
        // .or(other_route())
        .with(
            warp::cors()
                .allow_origin(dotenv!("APP_URL"))
                .allow_header("authorization")
                // Add other HTTP methods here
                .allow_methods(vec!["GET"]),
        )
        .with(warp::log("api"))
}

// A route to handle GET requests for a specific post
fn get_post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("posts" / i32)
        .and(warp::header::<String>("authorization"))
        .and(warp::get())
        .and_then(handlers::get_post)
}

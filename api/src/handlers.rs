use common;

// A function to handle GET requests at /posts/{id}
pub async fn get_post(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    let post = common::Post {
        id,
        title: String::from("Hello, T5!"),
        body: String::from("This is a response from the API."),
    };
    Ok(warp::reply::json(&post))
}

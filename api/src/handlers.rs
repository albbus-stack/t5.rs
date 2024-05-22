use crate::auth::jwt_valid;
use crate::db::{establish_connection, models::*};
use diesel::prelude::*;

// A function to handle GET requests at /posts/{id}
pub async fn get_post(post_id: i32, token: String) -> Result<impl warp::Reply, warp::Rejection> {
    use crate::db::schema::posts::dsl::*;

    if token.is_empty() {
        return Err(warp::reject());
    }

    let claims = jwt_valid(&token).await.unwrap();
    if claims.sub.is_empty() {
        return Err(warp::reject());
    }

    let connection = &mut establish_connection();
    let result = &posts
        .filter(id.eq(post_id))
        .limit(1)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts")[0];

    let post = common::Post {
        id: post_id,
        title: result.title.clone(),
        body: result.body.clone(),
    };
    Ok(warp::reply::json(&post))
}

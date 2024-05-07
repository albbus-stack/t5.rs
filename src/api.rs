use common::Post;

pub static API_URL: &str = "http://localhost:8000";

pub async fn get_post(id: i64) -> Result<Post, reqwest::Error> {
    let url = [API_URL, "posts", &id.to_string()].join("/");
    reqwest::get(&url).await?.json().await
}

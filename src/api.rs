use serde::{Deserialize, Serialize};

pub static API_URL: &str = "http://localhost:8000";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

pub async fn get_post(id: i64) -> Result<Post, reqwest::Error> {
    let url = [API_URL, "posts", &id.to_string()].join("/");
    reqwest::get(&url).await?.json().await
}

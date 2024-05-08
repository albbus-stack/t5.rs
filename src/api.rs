use common::Post;

pub async fn get_post(id: i64) -> Result<Post, reqwest::Error> {
    let url = [dotenv!("API_URL"), "posts", &id.to_string()].join("/");
    reqwest::get(&url).await?.json().await
}

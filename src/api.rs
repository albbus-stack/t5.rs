use common::Post;

pub async fn get_post(bearer_token: String, id: i64) -> Result<Post, reqwest::Error> {
    let url = [dotenv!("API_URL"), "posts", &id.to_string()].join("/");

    let client = reqwest::Client::builder().build().unwrap();

    let response = client
        .get(&url)
        .header(reqwest::header::AUTHORIZATION, bearer_token)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

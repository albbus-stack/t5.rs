use common::Post;
use std::collections::HashMap;

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

pub async fn get_authed_image(
    bearer_token: String,
    path: String,
) -> Result<String, reqwest::Error> {
    let image_url = format!(
        "{}/storage/v1/object/sign/{}",
        dotenv!("SUPABASE_URL").to_string(),
        path
    );
    let client = reqwest::Client::builder().build().unwrap();

    let mut map = HashMap::new();
    map.insert("expiresIn", "10");

    let response: serde_json::Value = client
        .post(&image_url)
        .json(&map)
        .header(reqwest::header::AUTHORIZATION, bearer_token)
        .send()
        .await?
        .json()
        .await?;

    Ok(response["signedURL"].as_str().unwrap_or("").to_string())
}

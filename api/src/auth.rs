use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

pub async fn jwt_valid(jwt: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = dotenv!("SUPABASE_JWT_SECRET");

    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);
    let decoded_token = decode::<Claims>(jwt, &decoding_key, &validation);

    match decoded_token {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => Err(err),
    }
}

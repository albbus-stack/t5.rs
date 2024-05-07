use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};

mod client;

#[derive(Clone, Debug)]
pub struct Supabase {
    client: Client,
    url: String,
    api_key: String,
    jwt_secret: String,
    bearer_token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshToken {
    refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

impl Clone for Claims {
    fn clone(&self) -> Self {
        Self {
            sub: self.sub.clone(),
            email: self.email.clone(),
            exp: self.exp,
        }
    }
}

impl Supabase {
    pub async fn jwt_valid(&self, jwt: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let secret = self.jwt_secret.clone();

        let decoding_key = DecodingKey::from_secret(secret.as_ref()).into();
        let validation = Validation::new(Algorithm::HS256);
        let decoded_token = decode::<Claims>(&jwt, &decoding_key, &validation);

        match decoded_token {
            Ok(token_data) => {
                println!("Token is valid. Claims: {:?}", token_data.claims);
                Ok(token_data.claims)
            }
            Err(err) => {
                println!("Error decoding token: {:?}", err);
                Err(err)
            }
        }
    }

    pub async fn sign_in_password(&self, email: &str, password: &str) -> Result<Response, Error> {
        let request_url: String = format!("{}/auth/v1/token?grant_type=password", self.url);
        let response: Response = self
            .client
            .post(&request_url)
            .header("apikey", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&Credentials {
                email: email.to_string(),
                password: password.to_string(),
            })
            .send()
            .await?;
        Ok(response)
    }

    // NOTE: This will fail unless you disable "Enable automatic reuse detection" in Supabase
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<Response, Error> {
        let request_url: String = format!("{}/auth/v1/token?grant_type=refresh_token", self.url);
        let response: Response = self
            .client
            .post(&request_url)
            .header("apikey", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&RefreshToken {
                refresh_token: refresh_token.to_string(),
            })
            .send()
            .await?;
        Ok(response)
    }

    pub async fn logout(&self) -> Result<Response, Error> {
        let request_url: String = format!("{}/auth/v1/logout", self.url);
        let token = self.bearer_token.clone().unwrap();
        let response: Response = self
            .client
            .post(&request_url)
            .header("apikey", &self.api_key)
            .header("Content-Type", "application/json")
            .bearer_auth(token)
            .send()
            .await?;
        Ok(response)
    }

    pub async fn signup_email_password(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Response, Error> {
        let request_url: String = format!("{}/auth/v1/signup", self.url);
        let response: Response = self
            .client
            .post(&request_url)
            .header("apikey", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&Credentials {
                email: email.to_string(),
                password: password.to_string(),
            })
            .send()
            .await?;
        Ok(response)
    }
}

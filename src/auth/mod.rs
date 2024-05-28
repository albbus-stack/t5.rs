use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};

mod client;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub bearer_token: String,
    pub refresh_token: String,
    pub email: String,
}

#[derive(Clone, Debug)]
pub struct Supabase {
    client: Client,
    url: String,
    api_key: String,
    pub user: Option<User>,
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
    pub async fn refresh_session(&self) -> Result<Response, Error> {
        let refresh_token = match self.user.clone() {
            Some(user) => user.refresh_token,
            None => "".to_string(),
        };
        let request_url: String = format!("{}/auth/v1/token?grant_type=refresh_token", self.url);
        let response: Response = self
            .client
            .post(&request_url)
            .header("apikey", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&RefreshToken {
                refresh_token: refresh_token,
            })
            .send()
            .await?;
        Ok(response)
    }

    pub async fn logout(&self) -> Result<Response, Error> {
        let request_url: String = format!("{}/auth/v1/logout", self.url);
        let token = match self.user.clone() {
            Some(user) => user.bearer_token,
            None => "".to_string(),
        };
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

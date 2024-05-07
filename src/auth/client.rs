use crate::auth::Supabase;
use reqwest::Client;

impl Supabase {
    pub fn new() -> Self {
        let client: Client = Client::new();
        let url: String = dotenv!("SUPABASE_URL").to_string();
        let api_key: String = dotenv!("SUPABASE_API_KEY").to_string();
        let jwt_secret: String = dotenv!("SUPABASE_JWT_SECRET").to_string();

        Supabase {
            client,
            url,
            api_key,
            jwt_secret,
            bearer_token: None,
        }
    }

    pub fn set_bearer_token(&mut self, token: String) {
        self.bearer_token = Some(token);
    }
}

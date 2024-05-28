use crate::auth::Supabase;
use reqwest::Client;

impl Supabase {
    pub fn new() -> Self {
        let client = Client::new();
        let url = dotenv!("SUPABASE_URL").to_string();
        let api_key = dotenv!("SUPABASE_API_KEY").to_string();

        Supabase {
            client,
            url,
            api_key,
            user: None,
        }
    }
}

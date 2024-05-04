use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

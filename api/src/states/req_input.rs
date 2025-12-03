use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WebsiteInput {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserInput {
    pub username: String,
    pub password: String,
}

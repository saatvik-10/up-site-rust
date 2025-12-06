use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WebsiteOutput {
    pub id: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserOutput {
    pub id: String,
    pub username: String
}

#[derive(Serialize, Deserialize)]
pub struct JwtOutput {
    pub jwt: String
}

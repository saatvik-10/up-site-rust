use dotenvy::dotenv;
use std::env::{self};

pub struct Config {
    pub db_url: String,
}

impl Default for Config {
    fn default() -> Self {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| panic!("Database ENV not found!"));

        Self { db_url }
    }
}

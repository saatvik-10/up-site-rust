use poem::{error::ResponseError, http::StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Err {
    #[error("User already exists")]
    UserExists,

    #[error("Invalid username or password")]
    InvalidCredentials,

    #[error("Database error: {0}")]
    Database(String),
}

impl ResponseError for Err {
    fn status(&self) -> StatusCode {
        match self {
            Err::UserExists => StatusCode::CONFLICT,
            Err::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Err::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn as_response(&self) -> poem::Response {
        poem::Response::builder()
            .status(self.status())
            .content_type("application/json")
            .body(format!(r#"{{"err": "{}"}}"#, self.to_string()))
    }
}

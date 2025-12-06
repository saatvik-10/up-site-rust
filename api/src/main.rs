use crate::handlers::{
    user::{user_sign_in, user_sign_up},
    website::{create_website, get_website},
};
use db::db::Db;
use poem::{EndpointExt, Route, Server, get, listener::TcpListener, post};
use std::sync::{Arc, Mutex};

pub mod handlers;
pub mod states;
pub mod err;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    let db = Arc::new(Mutex::new(Db::default().unwrap()));

    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/sign-up", post(user_sign_up))
        .at("/user/sign-in", post(user_sign_in))
        .data(db);

    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .run(app)
        .await
}

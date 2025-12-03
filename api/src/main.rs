use crate::states::{JwtOutput, UserInput, UserOutput, WebsiteInput, WebsiteOutput};
use db::db::Db;
use poem::{
    Route, Server, get, handler,
    listener::TcpListener,
    post,
    web::{self, Json, Path},
};

pub mod states;

#[handler]
fn create_website(Json(data): Json<WebsiteInput>) -> Json<WebsiteOutput> {
    // let url = data.url;
    let mut d = Db::default().unwrap();
    let website = d.create_website(String::from("1"), data.url).unwrap();

    let res = WebsiteOutput {
        id: website.id,
        url: website.url,
    };

    Json(res)
}

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("hello: {}", website_id)
}

#[handler]
fn user_sign_up(Json(data): Json<UserInput>) -> Json<UserOutput> {
    let mut d = Db::default().unwrap();
    let user = d.user_sign_up(data.username, data.password).unwrap();

    let res = UserOutput {
        id: user.id,
        username: user.username,
    };

    Json(res)
}

#[handler]
fn user_sign_in(Json(data): Json<UserInput>) -> Json<JwtOutput> {
    let mut d = Db::default().unwrap();
    let user = d.user_sign_up(data.username, data.password).unwrap();

    let res = JwtOutput {
        jwt: String::from("sm"),
    };

    Json(res)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website));

    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .run(app)
        .await
}

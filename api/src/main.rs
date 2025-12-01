use poem::{
    Route, Server, get, handler,
    listener::TcpListener,
    post,
    web::{Json, Path},
};

pub mod states;
use crate::states::{CreateWebsiteInput, CreateWebsiteOutput};

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    // let url = data.url;

    let res = CreateWebsiteOutput {
        // id: String::from("ID"),
        id: data.url,
    };

    Json(res)
}

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("hello: {}", website_id)
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

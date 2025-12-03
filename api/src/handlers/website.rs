use std::sync::{Arc, Mutex};

use crate::states::{WebsiteInput, WebsiteOutput};
use db::db::Db;
use poem::{
    handler,
    web::{Data, Json, Path},
};

#[handler]
pub fn create_website(
    Json(data): Json<WebsiteInput>,
    Data(db): Data<&Arc<Mutex<Db>>>,
) -> Json<WebsiteOutput> {
    let mut db_lock = db.lock().unwrap();
    let website = db_lock.create_website(String::from("1"), data.url).unwrap();

    let res = WebsiteOutput {
        id: website.id,
        url: website.url,
    };

    Json(res)
}

#[handler]
pub fn get_website(
    Path(website_id): Path<String>,
    Data(db): Data<&Arc<Mutex<Db>>>,
) -> Json<WebsiteOutput> {
    let mut db_lock = db.lock().unwrap();
    let website = db_lock.get_website(website_id).unwrap();

    let res = WebsiteOutput {
        id: website.id,
        url: website.url,
    };

    Json(res)
}

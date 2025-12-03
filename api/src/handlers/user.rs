use std::sync::{Arc, Mutex};

use crate::states::{JwtOutput, UserInput, UserOutput};
use db::db::Db;
use poem::{
    handler,
    web::{Data, Json},
};

#[handler]
pub fn user_sign_up(Json(data): Json<UserInput>, Data(db): Data<&Arc<Mutex<Db>>>) -> Json<UserOutput> {
    let mut db_lock = db.lock().unwrap();
    let user = db_lock.user_sign_up(data.username, data.password).unwrap();

    let res = UserOutput {
        id: user.id,
        username: user.username,
    };

    Json(res)
}

#[handler]
pub fn user_sign_in(Json(data): Json<UserInput>, Data(db): Data<&Arc<Mutex<Db>>>) -> Json<JwtOutput> {
    let mut db_lock = db.lock().unwrap();
    let _user = db_lock.user_sign_in(data.username, data.password).unwrap();

    let res = JwtOutput {
        jwt: String::from("sm"),
    };

    Json(res)
}
